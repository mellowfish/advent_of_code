import { Controller } from "@hotwired/stimulus"

export default class extends Controller {
  static targets = ['solutionPartOne', 'solutionPartTwo', 'preamble', 'epilogue'];
  static values = {
    dayUrl: String,
    examplesUrl: String,
    exampleUrl: String,
  };

  connect() {
    this.csrfToken = document.querySelector("[name='csrf-token']").content;
    this.solutionPartOne = this.solutionPartOneTarget.value;
    this.solutionPartTwo = this.solutionPartTwoTarget.value;
    this.preamble = this.preambleTarget.value;
    this.epilogue = this.epilogueTarget.value;
  }

  async update() {
    const oldSolutionPartOne = this.solutionPartOne;
    const oldSolutionPartTwo = this.solutionPartTwo;
    this.solutionPartOne = this.solutionPartOneTarget.value;
    this.solutionPartTwo = this.solutionPartTwoTarget.value;
    if(this.solutionPartOne === oldSolutionPartOne && this.solutionPartTwo === oldSolutionPartTwo) {
      return;
    }
    await this.save();
  }

  async save() {
    let formData = new FormData();
    formData.append("day[solution_part_one]", this.solutionPartOne);
    formData.append("day[solution_part_two]", this.solutionPartTwo);
    await fetch(
      this.dayUrlValue,
        {
          method: "PUT",
          body: formData,
          headers: {"X-CSRF-Token": this.csrfToken},
        }
      );
  }

  runTest(event) {
    const exampleNode = event.target.parentNode;
    const exampleId = exampleNode.dataset.exampleId;
    const part = exampleNode.querySelector('select[name="example[part]"]').value;
    const input = exampleNode.querySelector(`#test-${exampleId}-input`).value;
    let output = '';
    let debugOutput = '';
    if(part === 'one') {
      eval(this.preamble + this.solutionPartOne + this.epilogue + '\noutput = solve(input);\n');
    } else if (part === 'two') {
      eval(this.preamble + this.solutionPartTwo + this.epilogue + '\noutput = solve(input);\n');
    }
    const expectedOutput = exampleNode.querySelector(`#test-${exampleId}-expected-output`).value;
    exampleNode.querySelector(`#test-${exampleId}-actual-output`).value = output;
    exampleNode.querySelector(`#test-${exampleId}-debug`).value = debugOutput;

    if(output === expectedOutput) {
      exampleNode.classList.remove("IDE-example--selected");
      exampleNode.classList.remove("IDE-example--failure");
      exampleNode.classList.add("IDE-example--success");
    } else {
      exampleNode.classList.remove("IDE-example--selected");
      exampleNode.classList.remove("IDE-example--success");
      exampleNode.classList.add("IDE-example--failure");
    }
  }

  async saveTest(event) {
    const exampleNode = event.target.parentNode;
    const exampleId = exampleNode.dataset.exampleId;
    const input = exampleNode.querySelector(`#test-${exampleId}-input`).value;
    const part = exampleNode.querySelector('select[name="example[part]"]').value;
    console.log(part);
    const expectedOutput = exampleNode.querySelector(`#test-${exampleId}-expected-output`).value;

    if(exampleId === 'new') {
      let formData = new FormData();
      formData.append('example[input]', input);
      formData.append('example[expected_output]', expectedOutput);
      formData.append('example[part]', part);
      await fetch(
        this.examplesUrlValue,
        {
          method: "POST",
          body: formData,
          headers: {"X-CSRF-Token": this.csrfToken},
        }
      );
      document.location.reload(); // whatever
    } else {
      let formData = new FormData();
      formData.append("example[input]", input);
      formData.append("example[expected_output]", expectedOutput);
      formData.append('example[part]', part);
      await fetch(
        this.exampleUrlValue.replace("A113", exampleId),
        {
          method: "PUT",
          body: formData,
          headers: {"X-CSRF-Token": this.csrfToken},
        }
      );
    }
  }

  async deleteTest(event) {
    const exampleNode = event.target.parentNode;
    const exampleId = exampleNode.dataset.exampleId;
    await fetch(
      this.exampleUrlValue.replace("A113", exampleId),
      {
        method: "DELETE",
        headers: {"X-CSRF-Token": this.csrfToken},
      }
    );
    exampleNode.parentNode.removeChild(exampleNode);
  }

  selectTest(event) {
    document.querySelectorAll(".IDE-example--selected")
        .forEach((selectedExampleNode) => {
          selectedExampleNode.classList.remove("IDE-example--selected");
        });
    let exampleNode = event.target;
    while(!exampleNode.classList.contains("IDE-example")) {
      exampleNode = exampleNode.parentNode;
    }
    if(!exampleNode.classList.contains("IDE-example--success") && !exampleNode.classList.contains("IDE-example--failure")) {
      exampleNode.classList.add("IDE-example--selected");
    }
  }
}
