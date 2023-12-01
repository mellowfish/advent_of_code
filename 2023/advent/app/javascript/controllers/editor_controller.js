import { Controller } from "@hotwired/stimulus"

export default class extends Controller {
  static targets = ['solution', 'preamble', 'epilogue'];
  static values = {
    dayUrl: String,
    examplesUrl: String,
    exampleUrl: String,
  };

  connect() {
    this.csrfToken = document.querySelector("[name='csrf-token']").content;
    this.solution = this.solutionTarget.value;
    this.preamble = this.preambleTarget.value;
    this.epilogue = this.epilogueTarget.value;

    setTimeout(this.runCode.bind(this), 100);
  }

  async update() {
    const oldSolution = this.solution
    this.solution = this.solutionTarget.value;
    if(this.solution === oldSolution) {
      return;
    }
    this.runCode();
    await this.save();
  }

  async save() {
    let formData = new FormData();
    formData.append("day[solution]", this.solution);
    await fetch(
      this.dayUrlValue,
        {
          method: "PUT",
          body: formData,
          headers: {"X-CSRF-Token": this.csrfToken},
        }
      );
  }

  runCode() {
    eval(this.preamble + this.solution + this.epilogue);
  }

  runTest(event) {
    const exampleNode = event.target.parentNode;
    const exampleId = exampleNode.dataset.exampleId;
    const input = exampleNode.querySelector(`#test-${exampleId}-input`).value;
    var output = '';
    var debugOutput = '';
    eval(this.preamble + this.solution + this.epilogue + '\noutput = solve(input);\n');
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
    const expectedOutput = exampleNode.querySelector(`#test-${exampleId}-expected-output`).value;

    if(exampleId === 'new') {
      let formData = new FormData();
      formData.append("example[input]", input);
      formData.append("example[expected_output]", expectedOutput);
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
