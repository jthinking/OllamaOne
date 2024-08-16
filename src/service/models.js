import { Ollama } from "ollama/browser";
import { fetch } from "@tauri-apps/plugin-http";

export async function showModel(model) {
  const ollama_url = localStorage.getItem("ollama_url");
  const ollamaURL = ollama_url ? ollama_url : "http://localhost:11434";
  const ollama = new Ollama({
    host: ollamaURL,
    fetch: fetch,
  });
  try {
    const response = await ollama.show({ model: model });
    return response;
  } catch (error) {
    console.error(error);
    return undefined;
  }
}

export const MODEL_INFO_LIST = [
  {
    model: "llama2",
    title:
      "Llama 2 is a collection of foundation language models ranging from 7B to 70B parameters.",
    description:
      "Llama 2 is released by Meta Platforms, Inc. This model is trained on 2 trillion tokens, and by default supports a context length of 4096. Llama 2 Chat models are fine-tuned on over 1 million human annotations, and are made for chat.",
    logo: "",
    tags: [
      {
        name: "latest",
      },
      {
        name: "7b",
      },
      {
        name: "13b",
      },
      {
        name: "70b",
      },
    ],
  },
  {
    model: "llama3",
    title: "Meta Llama 3: The most capable openly available LLM to date",
    description:
      "Meta Llama 3, a family of models developed by Meta Inc. are new state-of-the-art , available in both 8B and 70B parameter sizes (pre-trained or instruction-tuned). Llama 3 instruction-tuned models are fine-tuned and optimized for dialogue/chat use cases and outperform many of the available open-source chat models on common benchmarks.",
    logo: "",
    tags: [
      {
        name: "latest",
      },
      {
        name: "8b",
      },
      {
        name: "70b",
      },
    ],
  },
  {
    model: "llama3.1",
    title: "Meta Llama 3: The most capable openly available LLM to date",
    description:
      "Meta Llama 3, a family of models developed by Meta Inc. are new state-of-the-art , available in both 8B and 70B parameter sizes (pre-trained or instruction-tuned). Llama 3 instruction-tuned models are fine-tuned and optimized for dialogue/chat use cases and outperform many of the available open-source chat models on common benchmarks.",
    logo: "",
    tags: [
      {
        name: "8b",
      },
      {
        name: "70b",
      },
      {
        name: "405b",
      },
    ],
  },
  {
    model: "mistral",
    title: "The 7B model released by Mistral AI, updated to version 0.2.",
    description:
      "Mistral is a 7.3B parameter model, distributed with the Apache license. It is available in both instruct (instruction following) and text completion.",
    logo: "",
    tags: [{ name: "latest" }, { name: "7b" }],
  },

  {
    model: "gemma",
    title:
      "Gemma is a family of lightweight, state-of-the-art open models built by Google DeepMind.",
    description:
      "Gemma is a new open model developed by Google and its DeepMind team. It’s inspired by Gemini models at Google.",
    logo: "",
    tags: [{ name: "latest" }, { name: "2b" }, { name: "7b" }],
  },

  {
    model: "gemma2",
    title:
      "Gemma is a family of lightweight, state-of-the-art open models built by Google DeepMind.",
    description:
      "Gemma is a new open model developed by Google and its DeepMind team. It’s inspired by Gemini models at Google.",
    logo: "",
    tags: [{ name: "latest" }, { name: "2b" }, { name: "9b" }, { name: "27b" }],
  },

  {
    model: "qwen",
    title:
      "Qwen 1.5 is a series of large language models by Alibaba Cloud spanning from 0.5B to 72B parameters",
    description:
      "Qwen is a series of transformer-based large language models by Alibaba Cloud, pre-trained on a large volume of data, including web texts, books, code, etc.",
    logo: "",
    tags: [
      { name: "latest", size: "1.51GB", description: "" },
      { name: "4b", size: "1.51GB", description: "" },
      { name: "7b", size: "2.89GB", description: "" },
      { name: "14b", size: "2.89GB", description: "" },
      { name: "72b", size: "2.89GB", description: "" },
    ],
  },

  {
    model: "qwen2",
    title:
      "Qwen2 is the large language model series developed by Qwen team, Alibaba Cloud.",
    description:
      "Qwen is a series of transformer-based large language models by Alibaba Cloud, pre-trained on a large volume of data, including web texts, books, code, etc.",
    logo: "",
    tags: [
      { name: "latest" },
      { name: "0.5b" },
      { name: "1.5b" },
      { name: "7b" },
      { name: "72b" },
    ],
  },

  {
    model: "yi",
    title: "A high-performing, bilingual language model.",
    description:
      "Yi is a series of large language models trained on a high-quality corpus of 3 trillion tokens that support both the English and Chinese languages.",
    logo: "",
    tags: [{ name: "latest" }, { name: "6b" }, { name: "9b" }, { name: "34b" }],
  },

  {
    model: "phi3",
    title:
      "Phi-3 Mini is a 3.8B parameters, lightweight, state-of-the-art open model by Microsoft.",
    description:
      "Phi-3 Mini is a 3.8B parameters, lightweight, state-of-the-art open model trained with the Phi-3 datasets that includes both synthetic data and the filtered publicly available websites data with a focus on high-quality and reasoning dense properties.",
    logo: "",
    tags: [{ name: "latest" }, { name: "3.8b" }, { name: "14b" }],
  },

  {
    model: "codellama",
    title:
      "A large language model that can use text prompts to generate and discuss code.",
    description:
      "Code Llama is a model for generating and discussing code, built on top of Llama 2. It’s designed to make workflows faster and efficient for developers and make it easier for people to learn how to code. It can generate both code and natural language about code. Code Llama supports many of the most popular programming languages used today, including Python, C++, Java, PHP, Typescript (Javascript), C#, Bash and more.",
    logo: "",
    tags: [
      { name: "latest" },
      { name: "7b" },
      { name: "13b" },
      { name: "34b" },
      { name: "70b" },
    ],
  },
  {
    model: "codegemma",
    title:
      "CodeGemma is a collection of powerful, lightweight models that can perform a variety of coding tasks like fill-in-the-middle code completion, code generation, natural language understanding, mathematical reasoning, and instruction following.",
    description:
      "CodeGemma is a collection of powerful, lightweight models that can perform a variety of coding tasks like fill-in-the-middle code completion, code generation, natural language understanding, mathematical reasoning, and instruction following.",
    logo: "",
    tags: [{ name: "latest" }, { name: "2b" }, { name: "7b" }],
  },
  {
    model: "codeqwen",
    title: "CodeQwen1.5 is the Code-Specific version of Qwen1.5.",
    description:
      "CodeQwen1.5 is the Code-Specific version of Qwen1.5. It is a transformer-based decoder-only language model pretrained on a large amount of data of codes.",
    logo: "",
    tags: [{ name: "latest" }, { name: "7b" }],
  },
];
