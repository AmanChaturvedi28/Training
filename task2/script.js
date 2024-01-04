function populateArray(arr, currentIndex, length) {
  if (currentIndex < length) {
    let userInput = prompt(`Enter element at index ${currentIndex}:`);

    if (userInput !== null) {
      arr[currentIndex] = userInput;
      populateArray(arr, currentIndex + 1, length);
    } else {
      console.log("User canceled input. Exiting.");
    }
  }
}

function displayArray(arr, currentIndex) {
  if (currentIndex < arr.length) {
    console.log(`Element at index ${currentIndex}: ${arr[currentIndex]}`);
    displayArray(arr, currentIndex + 1);
  }
}

let length = 5;
let myArray = new Array(length);

populateArray(myArray, 0, length);
displayArray(myArray, 0);

//factorial
function factorial(n) {
  if (n === 0 || n === 1) {
    return 1;
  } else {
    return n * factorial(n - 1);
  }
}

function nCr(n, r) {
  if (r === 0 || r === n) {
    return 1;
  } else {
    return factorial(n) / (factorial(r) * factorial(n - r));
  }
}

const n = 5;
const r = 2;
const result = nCr(n, r);
console.log(`C(${n}, ${r}) = ${result}`);
