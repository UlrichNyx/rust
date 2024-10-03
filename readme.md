RUST FOR THE WIN

Chapter 1: Getting Started
Challenge: Create a simple CLI application that asks for your name, then prints a custom greeting like "Hello, [name], welcome to the world of Rust!". Explore how to handle input/output and errors (e.g., if the user doesn’t enter anything).

Chapter 2: Programming a Guessing Game
Challenge: Extend the guessing game by adding a leaderboard. The game should keep track of multiple players’ scores (number of guesses they took to win) and display a leaderboard sorted from lowest to highest number of guesses.

Chapter 3: Common Programming Concepts
Challenge: Implement a basic temperature converter that can switch between Celsius and Fahrenheit. Allow the user to specify which direction they want to convert (e.g., Celsius to Fahrenheit or vice versa). Bonus points for handling invalid input gracefully.

Chapter 4: Understanding Ownership
Challenge: Write a function that takes a string and returns the length of the longest word in that string. Ensure the function handles ownership correctly—i.e., don’t transfer ownership of the string out of the function. Try using string slices to solve the problem.

Chapter 5: Using Structs to Structure Related Data
Challenge: Design a simple struct that represents a “Task” for a to-do list app. Each task should have a description, a status (completed or not), and an optional due date. Then, implement functionality to print out a list of tasks, showing their details.

Chapter 6: Enums and Pattern Matching
Challenge: Create an enum to represent different kinds of transport (e.g., Car, Bike, Bus, Walking) and write a function that returns the time it would take to travel a certain distance using that transport. Use pattern matching to implement the logic.

Chapter 7: Managing Growing Projects with Packages, Crates, and Modules
Challenge: Build a small Rust project with at least two modules. For example, you can create a "math" module for basic operations (add, subtract, multiply, divide), and a "greeting" module for personalized messages. Import these modules in your main program.

Chapter 8: Common Collections
Challenge: Write a function that finds the most frequent word in a given text. Use a HashMap to store the counts of each word, then determine which word appears the most frequently.

Chapter 9: Error Handling
Challenge: Implement a simple command-line calculator that can handle addition, subtraction, multiplication, and division. Use Result and Option to handle cases like dividing by zero or invalid input (non-numeric characters).

Chapter 10: Generic Types, Traits, and Lifetimes
Challenge: Create a generic function that finds the largest element in any list (e.g., numbers, characters). Make sure the function works with different types by using traits like PartialOrd. Add lifetime annotations where necessary to avoid ownership issues.

Chapter 11: Writing Automated Tests
Challenge: Write unit tests for the previous challenge (finding the largest element). Add test cases for edge conditions, such as an empty list or a list with one element. Then write integration tests for a more complex project, such as a to-do list app or a calculator.

Chapter 12: An I/O Project: Building a Command Line Program
Challenge: Extend the I/O project from the book by adding additional features:

Allow searching for multiple words.
Add a flag that makes the search case-insensitive.
Save the search results to a file instead of printing them.
