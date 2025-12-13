# Common Definitions

## Product, Production, Release

In a professional context, the concepts of a **product**, its **release**, and its **production** environment are fundamentally interdependent. A true product is not just the code itself, but the entire system of its creation, deployment, and use.

Let's define these interconnected elements:

* A **product** is a software application, system, tool, library, framework, platform, or service that is developed and delivered to end-users or customers.
* A **release** is the formal process that makes a version of the product available for deployment. It is the bridge between development and operation.
* A **production** environment is the target destination where the released product is deployed and operated for use by its end-users or customers.

These three elements create a complete lifecycle: a **product** is developed, it goes through a **release** process, and it is deployed into a **production** environment. The absence of any one of these elements signifies that we are not dealing with a product in this professional sense.

This distinction is crucial. For example, *Pet projects*, *Hobby projects*, *Learning projects*, and *Research projects* often lack one or more of these components. They might exist as code (a "product" in a loose sense) but are typically not intended for a formal **release** or sustained operation in a **production** environment.

There might be exceptions where such projects are eventually released and deployed, but these cases are not usual.

Therefore, for the purpose of this work, we consider such non-production projects to be out of scope, as they do not exhibit the complete lifecycle that defines a product in our context.

## Product Quality

Quality is difficult to define objectively. For example, the statement "there either is quality or not" is simple but incomplete.

Therefore, let us define quality subjectively. The most practical way to define quality is based on an individual's feeling, evaluation, appreciation, or judgment.

Let us establish common grades for **product quality**:

* **Resolved:** The product meets or exceeds all defined requirements and expectations, with no known significant issues or defects. It is stable, reliable, and performs well under expected conditions.
* **Contained:** The product meets most defined requirements and expectations, though it has some known issues or defects that do not significantly impact overall functionality or user experience. It is generally stable and reliable but may have minor limitations.
* **Unresolved:** The product is generally usable but fails to meet defined requirements and expectations. It has significant issues or defects that impact functionality, stability, or user experience, and may be unstable, unreliable, or incomplete.

The source of this judgment may vary depending on the development context.

Furthermore, a **product** typically has at least two perspectives for evaluation:

* **Internal evaluation:** Performed by the developer or development team.
* **External evaluation:** Performed by end-users, customers, or stakeholders.

Unless specified otherwise, all evaluations in this work are conducted from an internal perspective.

> In the scope of this project, we suppose quality includes all aspects of the product, including but not limited to functionality, performance, security, usability, maintainability, traceability, scalability, and compatibility and user experience.

## Product Costs

Cost is a vast topic, and while this document does not aim to be a cost-saving tool at present, cost remains a critical part of the development process.

Let us define common grades for **product costs** subjectively, based on an individual's evaluation:

* **Optimal**
* **Expensive**
* **Overwhelming**

As with quality, the source of judgment and the specific criteria for these grades are expected to differ across development contexts.

> In the scope of this project, we suppose costs includes all aspects of the development process, including but not limited to time, effort, resources, tools, infrastructure, and opportunity costs.

## Product Size

There is no objective definition for product sizes; therefore, it is more practical to define them on a relative scale.

**Product Sizes:**

* **Tiny:** The developer or team can manage all aspects of the project and deliver a product with **Resolved** quality and **Optimal** costs.
* **Small:** The developer or team struggles to manage all aspects but can still deliver the product with a trade-off between quality and cost (e.g., **Resolved** quality with **Expensive** costs, or **Contained** quality with **Optimal** costs).
* **Medium:** The developer or team cannot manage all aspects and can barely deliver the product, if at all. Possible outcomes include a trade-off between quality and cost (e.g., **Resolved** quality with **Overwhelming** costs, **Contained** quality with **Expensive** costs, or **Unresolved** quality with **Optimal** costs).
* **Large:** The developer or team is unable to manage all aspects of the project and cannot deliver a product with an acceptable combination of quality and cost (e.g., any pairing of **Resolved**/**Contained**/**Unresolved** quality with **Overwhelming**/**Expensive**/**Optimal** costs).
