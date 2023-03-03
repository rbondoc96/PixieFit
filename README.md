<h1 style="text-align: center;">Kratos - Workout Tracker</h1>

<p style="text-align: center;">A full-stack progressive web app (PWA) that aims to track workouts and user progression.</p>

## Possible Domain Names
* flexx.pxee.io

## Technologies Used
This polyrepo contains both the client and server code for the application. They are both developed using TypeScript.
* MongoDB - NoSQL database
* Express - Backend framework
* SolidJS - Frontend library
* Node.js - JS Runtime

In addition, the following technologies are also used:
* Vite - Frontend module bundler
* Mocha - Test runner
* Chai - Assertion library
* Sinon - Fakes, mocks, stubs, and spies

## Main Features
1. Workout Tracker
    - A tracker that can be used mid-workout to record exercises performed, the # of sets, and details of each set.
    - Has a timer feature to track rest periods between sets
2. Workout Diary
    - A collection of completed workouts
    - Shows various statistics compiled from the workouts

## Installation
From the project root, run the following commands to install the project's dependencies:
* `npm run client i` - Installs client dependencies
* `npm run server i` - Installs server dependencies

To run the development builds for each package:
* `npm run client run dev` - Run frontend development build
* `npm run server run dev` - Run backend development build

## Contributing
Feel free to open a new issue to discuss any bugs, ask for help, or feature requests.

Before contributing, please read the [standards](docs/standards.md) document.
