/*
!!! This is the old python code from the previous project !!!

import sqlite3
from datetime import datetime, timedelta
from enum import Enum

import pandas as pd


class HeaderIndex(Enum):
    NAME = 0
    CATEGORY = 1
    QUANTITY = 2
    COST = 3
    SALE_PRICE = 4
    AVAILABLE = 5
    DATE_STOCKED = 6
    CONTACT = 7
    QNT_SOLD = 8
    LOG_QNT_SOLD = 4

class DatabaseManager:
    DB_FILE = 'items.db'

    @staticmethod
    def connect():
        return sqlite3.connect(DatabaseManager.DB_FILE)

    @staticmethod
    def create_table(date=None):
        table_name = DatabaseManager.get_table_name(date)
        conn = DatabaseManager.connect()
        cursor = conn.cursor()
        cursor.execute(f'''
            CREATE TABLE IF NOT EXISTS {table_name} (
                id INTEGER PRIMARY KEY,
                name TEXT NOT NULL,
                category TEXT,
                quantity INTEGER DEFAULT 0,
                cost REAL DEFAULT 0,
                sale_price REAL DEFAULT 0,
                available BOOLEAN,
                date_stocked TEXT,
                contact TEXT,
                qnt_sold INTEGER DEFAULT 0
            )
        ''')
        conn.commit()
        conn.close()

    # Returns the name of
    @staticmethod
    def get_table_name(date=None):
        if date is None:
            date = datetime.now()
        return f"items_{date.strftime('%Y_%m_%d')}"


    # Returns the name of the most recently created table
    @staticmethod
    def get_most_recent_table():
        conn = DatabaseManager.connect()
        cursor = conn.cursor()
        cursor.execute("SELECT name FROM sqlite_master WHERE type='table' AND name LIKE 'items_%' ORDER BY name DESC LIMIT 1")
        result = cursor.fetchone()
        conn.close()
        return result[0] if result else None


    # Writes data from table widget to a daily table in database
    # Defaults to today's daily table
    @staticmethod
    def export_table(table, date=None):
        table_name = DatabaseManager.get_table_name(date)
        DatabaseManager.create_table(date)
        conn = DatabaseManager.connect()
        cursor = conn.cursor()
        data = DatabaseManager.read_table(table)

        # Clear existing table data before exporting
        cursor.execute(f'DELETE FROM {table_name}')

        for row in data:
            cursor.execute(f'''
                INSERT INTO {table_name} (name, category, quantity, cost, sale_price, available, date_stocked, contact, qnt_sold)
                VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)
            ''', row)

        conn.commit()
        conn.close()


    # Writes data from condensed sales log table to a daily table in database
    # Defaults to today's daily table
    @staticmethod
    def export_sales_log(table, date):
        table_name = DatabaseManager.get_table_name(date)
        conn = DatabaseManager.connect()
        cursor = conn.cursor()
        data = DatabaseManager.read_sales_log(table)

        # Update table with data
        try:
            for row in data:
                cursor.execute(f"UPDATE {table_name} SET qnt_sold = ? WHERE name = ?", (int(row[1]), row[0]))

        except sqlite3.OperationalError as e:
            print(f"An error occurred while saving sales log: {e}")

        finally:
            conn.commit()
            conn.close()


    # Reads data from a table widget's rows and returns a list
    @staticmethod
    def read_table(table):
        data = [[table.item(row, col).text() if table.item(row, col) else "" for col in range(table.columnCount())] for row in range(table.rowCount())]
        return data

    # Reads data from condensed sales log table and returns a list of items and qnt sold
    @staticmethod
    def read_sales_log(table):
        data = [[table.item(row, col).text() if table.item(row, col) else "" for col in [HeaderIndex.NAME.value, HeaderIndex.LOG_QNT_SOLD.value]] for row in range(table.rowCount())]
        return data


    # Queries database for most recent table or table matching specified date and returns table info in (headers, data) format
    @staticmethod
    def import_db(date=None):
        table_name = ""
        past_table = False
        if date is None:
            table_name = DatabaseManager.get_most_recent_table()
            # If most recent table is not today's table (i.e. today's table does not exist)
            if table_name: # Check if any table name was returned (i.e. any table exists)
                if datetime.strptime(table_name.removeprefix('items_'), "%Y_%m_%d").date() != datetime.now().date():
                    past_table = True
            else:
                table_name = DatabaseManager.get_table_name()  # Returns a table name with today's date
        else:
            table_name = DatabaseManager.get_table_name(date)

        conn = DatabaseManager.connect()
        cursor = conn.cursor()

        # Check if the table exists in the database
        cursor.execute(f"SELECT name FROM sqlite_master WHERE type='table' AND name='{table_name}'")
        table_exists = cursor.fetchone() is not None

        # Default headers and empty data to return if table does not exist
        default_headers = ['Name', 'Category', 'Quantity', 'Cost ($)', 'Sale Price ($)', 'Available', 'Date Stocked',
                           'Contact', 'Qnt Sold']
        data = []

        if not table_exists:
            # If the table does not exist, return headers and empty data
            conn.close()
            return default_headers, []

        # If the table exists, retrieve data from it
        cursor.execute(
            f"SELECT name, category, quantity, cost, sale_price, available, date_stocked, contact, qnt_sold FROM {table_name}")
        data = cursor.fetchall()
        # If table date is not specified, and most recent table is not today's table, set qnt_sold to 0
        if date is None and past_table:
            for row in data:
                list = [value for value in row] # Unpack tuple
                list[8] = 0
                data[data.index(row)] = tuple(list) # Assign new tuple to row

        conn.close()
        return default_headers, data


    @staticmethod
    def between_two_dates(start_date, end_date):
        table_name = DatabaseManager.get_table_name()
        conn = DatabaseManager.connect()
        query = f"""
            SELECT * FROM {table_name}
            WHERE strftime('%Y-%m-%d', date_stocked) BETWEEN '{start_date}' AND '{end_date}';
        """
        dataframe = pd.read_sql_query(query, conn)
        conn.close()
        return dataframe


    # Returns a list of database table names matching the "item_yyyy_MM_dd" format within time span
    @staticmethod
    def tables_between_dates(from_date, to_date):
        conn = DatabaseManager.connect()
        cursor = conn.cursor()

        # Get all tables with "item_yyyy_MM_dd" name format
        table_prefix = "items_"
        cursor.execute(f"SELECT name FROM sqlite_master WHERE type='table' AND name LIKE '{table_prefix}%'")
        table_names = [name[0] for name in cursor.fetchall()]

        # Remove table names outside of time span from list
        filtered_names = []
        for table_name in table_names:
            try:
                name_date = table_name.removeprefix(table_prefix)
                name_date = datetime.strptime(name_date, "%Y_%m_%d").date()

                if from_date <= name_date <= to_date:
                    filtered_names.append(table_name)

            except ValueError:  # Skip any table names that can't be converted to a date
                continue

        conn.close()
        return filtered_names


    # Returns a list of unique item names from all item tables within time span
    @staticmethod
    def items_between_dates(from_date, to_date):
        conn = DatabaseManager.connect()
        cursor = conn.cursor()

        table_names = DatabaseManager.tables_between_dates(from_date, to_date)

        # Get unique item names from all tables
        item_names = set()
        for table_name in table_names:
            cursor.execute(f"SELECT DISTINCT name FROM {table_name}")

            for item_name in cursor.fetchall():
                item_names.add(item_name[0])

        conn.close()
        return item_names


    # Returns a list with the name, category, sale_price, and qnt_sold for each item in the list of item names
    @staticmethod
    def item_sales_data(item_names, date=None):
        table_name = DatabaseManager.get_table_name(date)
        conn = DatabaseManager.connect()
        cursor = conn.cursor()

        try:
            # For all items in item_names, get name, sale_price, and qnt_sold
            query = f"""
                SELECT name, category, sale_price, qnt_sold
                FROM {table_name}
                WHERE name IN ({', '.join('?' for _ in item_names)})
            """

            cursor.execute(query, item_names)
            results = cursor.fetchall()

            if not results:
                results = [[name, "Misc.", 0, 0] for name in item_names]

        # If table does not exist or other error occurs during query, default 0 sale_price and 0 qnt_sold
        except sqlite3.OperationalError as e:
            print(f"An error occurred while fetching item data: {e}")
            results = [[name, "Misc.", 0, 0] for name in item_names]

        finally:
            conn.close()

        return results


    # Returns a tuple of two lists of dictionaries, (sales_data_by_item, gross_category_sales)
    @staticmethod
    def sales_between_dates(item_names, start_date, end_date):
        # Initialize a dictionary to hold sales data for each item
        sales_data_by_item = {name: {'name': name, 'category': "Misc.", 'sale_price': 0, 'daily_sales': []} for name in
                              item_names}

        # Initialize a dictionary to hold gross sales by category
        gross_category_sales = {}

        # Loop through each day between start_date and end_date
        current_date = start_date
        sale_price = 0
        while current_date <= end_date:
            # Get sales data for the current day
            daily_sales_data = DatabaseManager.item_sales_data(item_names, current_date)

            # Process daily sales data
            for item_data in daily_sales_data:
                name = item_data[0]
                category = item_data[1]
                sale_price = max(item_data[2], sale_price) # Accept any non-zero sales_price as new value
                qnt_sold = item_data[3]

                # Update the dictionary with daily sales
                if name in sales_data_by_item:
                    # Accept any non-default category as new value
                    current_category = sales_data_by_item[name]['category']
                    if current_category == "Misc." and category != "Misc.":
                        sales_data_by_item[name]['category'] = category  # Update category

                    sales_data_by_item[name]['sale_price'] = sale_price  # Update sale_price
                    sales_data_by_item[name]['daily_sales'].append(
                        {'date': current_date.strftime("%Y-%m-%d"), 'qnt_sold': qnt_sold})

                    # Calculate gross sales and add to category's gross sales
                    gross = float(qnt_sold) * sale_price
                    if category in gross_category_sales:
                        gross_category_sales[category] += gross
                    else:
                        gross_category_sales[category] = gross

            # Move to the next day
            current_date += timedelta(days=1)

        # Return the sales data and gross category sales as lists of dictionaries
        return list(sales_data_by_item.values()), gross_category_sales

    @staticmethod
    def sales_data(dataframe):
        grouped_df = dataframe.groupby('name').agg({
            'cost': list,
            'quantity': list,
            'sale_price': list,
            'date_stocked': list,
        }).reset_index()
        return grouped_df

*/