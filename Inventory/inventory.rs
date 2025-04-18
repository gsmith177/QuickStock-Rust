/*
!!! This is the old python code from the previous project !!!

from datetime import datetime

from PySide6.QtCore import QRect, Qt, QDate, QSize
from PySide6.QtGui import QColor, QIcon
from PySide6.QtWidgets import QHeaderView, QWidget, QTableWidgetItem, QTableWidget, QVBoxLayout, QDateEdit, QSpacerItem, \
    QSizePolicy, QHBoxLayout, QPushButton, QCheckBox

import popups
from tablereader import DatabaseManager, HeaderIndex
from widgetdesigners import Ui_InventoryWidget


# Screen user sees when viewing/making changes to inventory
class InventoryScreen(QWidget, Ui_InventoryWidget):

    def __init__(self, show_home_screen):
        # Init ui
        super(InventoryScreen, self).__init__()
        self.setupUi(self)
        self.setup_table()
        self.searchKeyBar.textChanged.connect(self.searchKeyBar_textChanged)
        self.searchCategoryBar.textChanged.connect(self.searchCategoryBar_textChanged)

        # Connect buttons
        self.addItemButton.clicked.connect(self.add_table_row)
        self.editItemButton.clicked.connect(self.editItemButton_clicked)
        self.removeItemButton.clicked.connect(self.removeItemButton_clicked)
        self.saveButton.clicked.connect(self.saveButton_clicked)
        self.homeButton.clicked.connect(show_home_screen)  # Determines action by slot passed in constructor
        self.searchKeyBar.textChanged.connect(self.search_table)  # Connect search bars to the search function
        self.searchCategoryBar.textChanged.connect(self.search_table)

        # Creates daily table in database
        # Useful for sales log
        DatabaseManager.export_table(self.tableWidget)

    # Imports data from an existing table into tableWidget
    def import_table(self):
        # Clear pre-set columns
        self.tableWidget.setColumnCount(0)

        table = DatabaseManager.import_db()
        # Add columns titles
        for column in table[0]:
            self.add_table_column(column)

        # Add row data
        for row in table[1]:
            self.add_table_row(row)

    # Set table in default state
    def setup_table(self):
        # Import table data
        self.import_table()
        self.disable_table_editing()

        # Set table formatting
        self.tableWidget.horizontalHeader().setSectionResizeMode(QHeaderView.Stretch)
        self.tableWidget.setSelectionBehavior(QTableWidget.SelectRows)


    def reset_table(self):
        self.tableWidget.setRowCount(0)
        self.setup_table()


    # Disables editing of all rows and columns in table
    def disable_table_editing(self):
        for row in range(self.tableWidget.rowCount()):
            for column in range(self.tableWidget.columnCount()):
                item = self.tableWidget.item(row, column)
                if item is None:  # If the cell is blank, create a new item
                    item = QTableWidgetItem("")
                    self.tableWidget.setItem(row, column, item)
                item.setFlags(item.flags() & ~Qt.ItemIsEditable) # Removes editable flag from item (row, column)


    # Enables editing of selected row in table
    def enable_table_row_editing(self, row):
        for column in range(self.tableWidget.columnCount()):
            item = self.tableWidget.item(row, column)
            if item is None:  # If the cell is blank, create a new item
                item = QTableWidgetItem("")
                self.tableWidget.setItem(row, column, item)
            item.setFlags(item.flags() | Qt.ItemIsEditable) # Adds editable flag to item (row, column)
            item.setBackground(QColor("light blue")) # Mark edited row with blue


    # Adds new row to table and enables editing of row
    # row_data is an optional parameter to set values of row
    def add_table_row(self, row_data=None):
        # Create new row
        row = self.tableWidget.rowCount()
        self.tableWidget.insertRow(row)

        # If row_data is provided, use its values; otherwise, default to "New Item" and blank spaces
        for column in range(self.tableWidget.columnCount()):
            isImport = False
            # Determine if column is "Available" column
            if column == HeaderIndex.AVAILABLE.value:
                checkbox = QCheckBox()
                if row_data and column < len(row_data):
                    isImport = True
                    checkbox.setChecked(row_data[HeaderIndex.QUANTITY.value] > 0)
                else:
                    checkbox.setChecked(False)

                # Add layout to center checkbox in cell
                widget = QWidget()
                layout = QVBoxLayout(widget)
                layout.addWidget(checkbox)
                layout.setAlignment(Qt.AlignCenter)
                self.tableWidget.setCellWidget(row, column, widget)

            else:
                if row_data and column < len(row_data):
                    isImport = True
                    item_data = row_data[column]
                    if isinstance(item_data, (int, float)): # Convert numbers to strings
                        item_data = str(item_data)
                else:
                    item_data = "New Item" if column == 0 else ""

                # Add item to table
                item = QTableWidgetItem(item_data)
                self.tableWidget.setItem(row, column, item)
                if not isImport: item.setBackground(QColor("light green")) # Mark empty added row with green


    # Adds new column to table
    # Used for overwriting default columns with file
    def add_table_column(self, column_name=""):
        # Create new column
        column_index = self.tableWidget.columnCount()
        self.tableWidget.insertColumn(column_index)

        # Add column name
        item = QTableWidgetItem(column_name)
        self.tableWidget.setHorizontalHeaderItem(column_index,  item)


    # Removes selected row from table
    def remove_table_row(self, row):
        self.tableWidget.removeRow(row)


    # Enables editing of selected rows when user clicks editItemButton
    def editItemButton_clicked(self):
        for index in self.tableWidget.selectedIndexes():
            self.enable_table_row_editing(index.row())


    # Removes selected rows when user clicks removeItemButton
    def removeItemButton_clicked(self):
        if popups.delete_confirmation_dialog():
            #Get row indexes in reverse order to avoid changing indexes as rows are removed
            selected_rows = sorted(set(index.row() for index in self.tableWidget.selectedIndexes()), reverse=True)

            for row in selected_rows:
                self.remove_table_row(row)


    # Checks if table widget's data can be saved in valid format
    # Returns an error message with False cases
    def valid_save(self):
        try:
            item_names = set()

            for row in range(self.tableWidget.rowCount()):
                item_name = self.tableWidget.item(row, HeaderIndex.NAME.value).text()

                # Check if the item name is empty or set to a placeholder value
                if item_name in ["", "New Item"]:
                    return False, "Item names must be set before saving."

                # Check for duplicate item names
                if item_name in item_names:
                    return False, f"Duplicate item name found: '{item_name}'. \nEach item must have a unique name."
                item_names.add(item_name)

                # Validate numerical values in specific columns
                for col in [HeaderIndex.COST, HeaderIndex.SALE_PRICE, HeaderIndex.QUANTITY]:
                    float(self.tableWidget.item(row, col.value).text())

        except ValueError:
            return False, "File cannot be saved if values in Quantity, Cost, or Sale Price columns are not numerical values."

        return True, ""


    # Saves table widget's data to file
    def saveButton_clicked(self):
        save_test = self.valid_save()
        if save_test[0]:
            # Set default values for qnt_sold and available
            for row in range(self.tableWidget.rowCount()):
                if self.tableWidget.item(row, HeaderIndex.QNT_SOLD.value).text() == "":
                    self.tableWidget.item(row, HeaderIndex.QNT_SOLD.value).setText("0")
                available = int(self.tableWidget.item(row, HeaderIndex.QUANTITY.value).text()) > 0
                #self.tableWidget.item(row, HeaderIndex.AVAILABLE.value).setText(str(available))

            DatabaseManager.export_table(self.tableWidget)
            self.reset_table()
        else:
            popups.error_dialog(save_test[1], "Save Error")


    def searchKeyBar_textChanged(self):
        # Changes formatting for placeholder vs. user text
        if not self.searchKeyBar.text() == "":
            # Remove italics from user text
            font = self.searchKeyBar.font()
            font.setItalic(False)
            self.searchKeyBar.setFont(font)
        else:
            # Add italics to placeholder text
            font = self.searchKeyBar.font()
            font.setItalic(True)
            self.searchKeyBar.setFont(font)

        self.search_table()


    def searchCategoryBar_textChanged(self):
        # Changes formatting for placeholder vs. user text
        if not self.searchCategoryBar.text() == "":
            # Remove italics from user text
            font = self.searchCategoryBar.font()
            font.setItalic(False)
            self.searchCategoryBar.setFont(font)
        else:
            # Add italics to placeholder text
            font = self.searchCategoryBar.font()
            font.setItalic(True)
            self.searchCategoryBar.setFont(font)


    # Searches tableWidget for item names containing user-input string
    def search_table(self):
        category_search = self.searchCategoryBar.text().lower()
        value_search = self.searchKeyBar.text().lower()

        # Step 1: Identify target column based on category input
        category_column_index = -1
        if category_search:
            for column in range(self.tableWidget.columnCount()):
                header_item = self.tableWidget.horizontalHeaderItem(column)
                if header_item and category_search in header_item.text().lower():
                    category_column_index = column
                    break

        matching_rows = set()

        # Step 2: Filter rows based on category column if specified
        if category_column_index != -1:
            # Search only within specific category column
            for row in range(self.tableWidget.rowCount()):
                item = self.tableWidget.item(row, category_column_index)
                if item and value_search in item.text().lower():
                    matching_rows.add(row)
        else:
            # Else, search for value across all columns
            for row in range(self.tableWidget.rowCount()):
                for column in range(self.tableWidget.columnCount()):
                    item = self.tableWidget.item(row, column)
                    if item and value_search in item.text().lower():
                        matching_rows.add(row)
                        break  # Move to next row as soon as a match is found

        # Step 3: Show or hide rows based on final filtered rows
        for row in range(self.tableWidget.rowCount()):
            if row in matching_rows:
                self.tableWidget.showRow(row)
            else:
                self.tableWidget.hideRow(row)

    # Event triggered by window resize, adjusts size and position of widgets
    def resizeEvent(self, event):
        # Get the current window width and height
        window_width = self.width()
        window_height = self.height()

        # Resize and position the table widget dynamically
        self.tableWidget.setGeometry(QRect(10, 60, window_width - 20, window_height - 120))

        # Resize and position the buttons dynamically
        button_width = 87
        button_height = 26

        # Button positions
        button_x_start = window_width - button_width - 10  # Adjust the X position to stay on the right side
        searchBarGap = self.searchCategoryBar.geometry().right() - self.searchKeyBar.geometry().left() + 20
        categoryBarRight = self.searchCategoryBar.geometry().right()
        searchBarLength =  self.tableWidget.geometry().right() - self.searchKeyBar.geometry().left()

        # Set positions for each button
        # Add, edit, and remove buttons on the bottom-right
        self.addItemButton.setGeometry(QRect(button_x_start - 200, window_height - 40, button_width, button_height))
        self.editItemButton.setGeometry(QRect(button_x_start - 100, window_height - 40, button_width, button_height))
        self.removeItemButton.setGeometry(QRect(button_x_start, window_height - 40, button_width, button_height))
        self.saveButton.setGeometry(QRect(10, window_height - 40, 71, button_height))  # Save button on the bottom-left

        # Resize and position the search bar and buttons at the top
        self.searchKeyBar.setGeometry(QRect(categoryBarRight + searchBarGap, 20, searchBarLength, button_height))  # Search bar on the top-right

        # Call the parent class resizeEvent to ensure proper handling
        super().resizeEvent(event)


# Condensed version of inventory table used to log quantity of items sold
# Meant to be accessed only by sales screen and only return to sales screen
class SalesLogScreen(QWidget):
    def __init__(self, show_sales_screen):
        super().__init__()
        self.main_layout = QVBoxLayout()
        self.setLayout(self.main_layout)
        self.show_sales_screen = show_sales_screen # Save slot as function

        self.top_layout = QHBoxLayout()
        top_spacer = QSpacerItem(40, 40, QSizePolicy.Expanding, QSizePolicy.Minimum)
        self.top_layout.addItem(top_spacer)
        self.date_edit = QDateEdit()
        self.date_edit.setDisplayFormat("yyyy-MM-dd")
        self.date_edit.setCalendarPopup(True)
        self.date_edit.setDate(QDate.currentDate())
        self.top_layout.addWidget(self.date_edit)
        self.main_layout.addLayout(self.top_layout)

        self.tableWidget = QTableWidget()
        self.setup_table()
        self.main_layout.addWidget(self.tableWidget)

        self.bottom_layout = QHBoxLayout()
        icon = QIcon()
        icon.addFile(u"icons/save.svg", QSize(), QIcon.Mode.Normal, QIcon.State.Off)
        self.save_button = QPushButton(icon, "Save", self)
        self.save_button.clicked.connect(self.save_clicked)
        self.bottom_layout.addWidget(self.save_button)
        bottom_spacer = QSpacerItem(40, 40, QSizePolicy.Expanding, QSizePolicy.Minimum)
        self.bottom_layout.addItem(bottom_spacer)
        icon.addFile(u"icons/x-square.svg", QSize(), QIcon.Mode.Normal, QIcon.State.Off)
        self.cancel_button = QPushButton(icon, "Cancel", self)
        self.cancel_button.clicked.connect(self.cancel_clicked)
        self.bottom_layout.addWidget(self.cancel_button)
        self.main_layout.addLayout(self.bottom_layout)


    def setup_table(self):
        # Import table data
        self.import_table()

        # Set table formatting
        self.tableWidget.horizontalHeader().setSectionResizeMode(QHeaderView.Stretch)


    # Imports items' name, category, and price
    def import_table(self):
        table = DatabaseManager.import_db()

        # Get the index of the required columns
        name = HeaderIndex.NAME.value
        category = HeaderIndex.CATEGORY.value
        cost = HeaderIndex.COST.value
        price = HeaderIndex.SALE_PRICE.value
        qnt_sold = HeaderIndex.QNT_SOLD.value

        # Create rows and columns
        self.tableWidget.setRowCount(len(table[1]))
        self.tableWidget.setColumnCount(5)
        self.tableWidget.setHorizontalHeaderLabels(["Name", "Category", "Cost ($)", "Sale Price ($)", "Quantity Sold"])

        # Add row data, lock "Name", "Category", "Cost". and "Sale Price" columns
        for row_index, row in enumerate(table[1]):
            name_cell = QTableWidgetItem(row[name])
            name_cell.setFlags(name_cell.flags() & ~Qt.ItemIsEditable)
            self.tableWidget.setItem(row_index, 0, name_cell)

            category_cell = QTableWidgetItem(row[category])
            category_cell.setFlags(category_cell.flags() & ~Qt.ItemIsEditable)
            self.tableWidget.setItem(row_index, 1, category_cell)

            cost_cell = QTableWidgetItem(str(row[cost]))
            cost_cell.setFlags(cost_cell.flags() & ~Qt.ItemIsEditable)
            self.tableWidget.setItem(row_index, 2, cost_cell)

            price_cell = QTableWidgetItem(str(row[price]))
            price_cell.setFlags(price_cell.flags() & ~Qt.ItemIsEditable)
            self.tableWidget.setItem(row_index, 3, price_cell)

            quantity_sold_cell = QTableWidgetItem(str(row[qnt_sold]))
            self.tableWidget.setItem(row_index, 4, quantity_sold_cell)


    # Checks if table's data can be saved in valid format
    def valid_save(self):
        col = HeaderIndex.LOG_QNT_SOLD.value
        try:
            # Check if any cell in the Quantity Sold column is not an integer
            for row in range(self.tableWidget.rowCount()):
                int(self.tableWidget.item(row, col).text())
        except ValueError:
            return False, "Quantity Sold must be a whole number."


        return True, ""


    # Exports data from table, then switches back to sales screen
    def save_clicked(self):
        save_test = self.valid_save()
        # Check if save data is in valid format (only enforces numerical value fields)
        if save_test[0]:
            # Confirm save
            if popups.save_confirmation_dialog():
                DatabaseManager.export_sales_log(self.tableWidget, datetime.strptime(self.date_edit.text(), "%Y-%m-%d"))
                self.show_sales_screen()
                del self # Deletes instance of SalesLogScreen
        else:
            popups.error_dialog(save_test[1], "Save Error")


    # Discards table data on confirmation and switches back to sales screen
    def cancel_clicked(self):
        if popups.cancel_confirmation_dialog():
            self.show_sales_screen()
            del self # Deletes instance of SalesLogScreen

*/