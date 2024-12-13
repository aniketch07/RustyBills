# RustyBills

**RustyBills** is a secure and efficient bill management system built in Rust. It enables users to manage their bills by adding, viewing, updating, and removing bills. RustyBills also features a user authentication system with bcrypt password hashing and securely stores bill data in a JSON file.

## Features

- **Add Bills**: Easily add new bills with a name and amount.
- **View Bills**: List all saved bills with their details.
- **Update Bills**: Modify existing bills with updated amounts.
- **Remove Bills**: Delete bills from the system.
- **Data Persistence**: Save and load bills to/from a JSON file for data persistence.

## Getting Started

### Prerequisites
- Install [Rust](https://www.rust-lang.org/) on your system.

### Installation
1. Clone the repository:
   ```bash
   git clone <repository-url>
   cd RustyBills
   ```
2. Install dependencies:
   ```bash
   cargo build
   ```
3. Run the application:
   ```bash
   cargo run
   ```

## Usage

When you run the application, you will see the following menu:

```
==Bill Manager==
1. Add Bill
2. View Bills
3. Remove Bills
4. Update Bill
Enter Selection:
```

### Adding a Bill
1. Select option `1`.
2. Enter the bill name and amount when prompted.

### Viewing Bills
1. Select option `2` to view all saved bills.

### Removing a Bill
1. Select option `3`.
2. Enter the name of the bill you wish to remove.

### Updating a Bill
1. Select option `4`.
2. Enter the name of the bill and the new amount.


## File Structure
```
RustyBills/
├── src/
│   ├── main.rs          # Main program logic
├── bills.json           # JSON file for storing bills
├── Cargo.toml           # Rust package configuration
```

## Future Enhancements
- Multi-user support with separate bill files for each user.
- Command-line arguments for faster operations.
- Integration with a database for better scalability.
- Enhanced UI using a terminal library like `crossterm`.

## License
This project is licensed under the MIT License.

---

Feel free to contribute to this project and enhance its functionality! Pull requests are welcome.
