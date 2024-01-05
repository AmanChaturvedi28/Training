function generateDigitPermutations(input) {
  const strInput = input.toString();
  const result = [];

  function permute(current, remainingDigits) {
    if (current.length > 0) {
      result.push(parseInt(current));
    }

    if (remainingDigits.length === 0) {
      return;
    }

    for (let i = 0; i < remainingDigits.length; i++) {
      const updatedCurrent = current + remainingDigits[i];
      const updatedRemaining =
        remainingDigits.slice(0, i) + remainingDigits.slice(i + 1);
      permute(updatedCurrent, updatedRemaining);
    }
  }

  permute("", strInput);

  const uniquePermutations = [...new Set(result)];
  return uniquePermutations;
}

function isPrime(num) {
  if (num <= 1) {
    return false;
  }
  for (let i = 2; i <= Math.sqrt(num); i++) {
    if (num % i === 0) {
      return false;
    }
  }
  return true;
}

function generatePascalsTriangle(rows, maxPrime) {
  const triangle = [];
  for (let i = 0; i < rows; i++) {
    triangle[i] = new Array(i + 1);
    triangle[i][0] = 1;
    triangle[i][i] = 1;
    for (let j = 1; j < i; j++) {
      triangle[i][j] = triangle[i - 1][j - 1] + triangle[i - 1][j];
      if (triangle[i][j] > maxPrime) {
        return triangle.slice(0, i);
      }
    }
  }
  return triangle;
}

document.getElementById("submitButton").addEventListener("click", function () {
  let inputNumber = parseInt(document.getElementById("inputNumber").value);

  if (isNaN(inputNumber) || inputNumber < 1000) {
    alert("Please enter a valid number that is at least 1000.");
    return;
  }

  let maxPrime = -1;

  while (!isPrime(inputNumber)) {
    inputNumber--;
  }

  maxPrime = inputNumber;
  document.getElementById("inputNumber").value = inputNumber;

  document.getElementById("result").innerText = "Max Prime Number: " + maxPrime;

  const pascalsTriangle = generatePascalsTriangle(maxPrime, maxPrime);

  const pascalDisplay = document.getElementById("pascalTriangle");
  pascalDisplay.innerHTML = "";

  const primeNumbersInTriangle = [];
  for (let i = 0; i < pascalsTriangle.length; i++) {
    const row = document.createElement("div");
    row.classList.add("row");

    for (let j = 0; j < pascalsTriangle[i].length; j++) {
      const cell = document.createElement("span");

      const currentNumber = pascalsTriangle[i][j];

      if (
        isSingleDigitPrime(currentNumber) &&
        inputNumber.toString().includes(currentNumber.toString())
      ) {
        cell.classList.add("prime");
        primeNumbersInTriangle.push(currentNumber);
      }

      const spaces = 4;
      cell.innerHTML = currentNumber + "&nbsp;".repeat(spaces);
      row.appendChild(cell);
    }

    pascalDisplay.appendChild(row);
  }

  const allPrimeNumbersInTriangle = generateDigitPermutations(
    maxPrime.toString()
  ).filter((number) => isPrime(number));

  const primeNumbers = generateDigitPermutations(maxPrime.toString()).filter(
    (number) => isPrime(number)
  );

  const errorRatio =
    (1 - primeNumbersInTriangle.length / allPrimeNumbersInTriangle.length) *
    100;
  document.getElementById("primeArray").innerText =
    "Prime Numbers: " + primeNumbers.join(", ");

  document.getElementById(
    "errorRatio"
  ).innerText = `Error Ratio: ${errorRatio.toFixed(2)}%`;

  const notIncludedPrimeNumbers = allPrimeNumbersInTriangle.filter(
    (number) => !primeNumbersInTriangle.includes(number)
  );

  document.getElementById("notIncludedPrime").innerText =
    "Not Included Primes: " + notIncludedPrimeNumbers.join(", ");
});

function isSingleDigitPrime(num) {
  return num < 10 && isPrime(num);
}
