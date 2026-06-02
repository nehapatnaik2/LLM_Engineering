# LLM Engineering — Projects Portfolio

A collection of projects from the **LLM Engineering** course (weeks 1–8) plus personal experiments. Each week builds on the last, ending with an autonomous agentic system in Week 8.

> **Repo layout:** Course material lives in `week1/` … `week8/` at the repository root. 

---

## Course projects by week

### Week 1 — APIs, scraping & first business app

| Project | Location | Description |
|---------|----------|-------------|
| **Your first lab** | [`week1/day1.ipynb`](../week1/day1.ipynb) | Connect to OpenAI (and Ollama); first calls to frontier models. |
| **OpenAI client & Ollama** | [`week1/day2.ipynb`](../week1/day2.ipynb) | OpenAI Python SDK, local models, and comparing providers. |
| **Web scraper** | [`week1/scraper.py`](../week1/scraper.py) | Fetch and extract text from company websites. |
| **Company brochure generator** | [`week1/day5.ipynb`](../week1/day5.ipynb) | Scrape a company site and generate a marketing brochure with an LLM. |
| **End-of-week exercise** | [`week1/week1 EXERCISE.ipynb`](../week1/week1%20EXERCISE.ipynb) | Technical Q&A tool: answer coding questions with explanations (OpenAI or Ollama). |

---

### Week 2 — Gradio, multi-model & tools

| Project | Location | Description |
|---------|----------|-------------|
| **Multi-model chat** | [`week2/day1.ipynb`](../week2/day1.ipynb) – [`day4.ipynb`](../week2/day4.ipynb) | OpenAI-compatible clients for Gemini, Groq, DeepSeek, Ollama; streaming and system prompts. |
| **Airline AI assistant** | [`week2/day5.ipynb`](../week2/day5.ipynb) | Gradio chatbot with tools, streaming, and multimodal (image) support. |
| **End-of-week exercise** | [`week2/week2 EXERCISE.ipynb`](../week2/week2%20EXERCISE.ipynb) | Full Gradio prototype of the Week 1 Q&A tool with model switching and optional audio. |

---

### Week 3 — Open-source models & synthetic data

| Project | Location | Description |
|---------|----------|-------------|
| **HuggingFace pipelines** | [`week3/day2.ipynb`](../week3/day2.ipynb) | Colab: transformers pipelines on GPU (see notebook for Colab link). |
| **Tokenizers** | [`week3/day3.ipynb`](../week3/day3.ipynb) | How tokenization affects cost and context. |
| **Synthetic data generator** | [`week3/generate_synthetic_data.ipynb`](../week3/generate_synthetic_data.ipynb) | Gradio app to generate structured datasets with LLMs. |
| **Meeting minutes creator** | [`week3/day5.ipynb`](../week3/day5.ipynb) | Transcribe and summarize meetings with open-source + frontier models. |

---

### Week 4 — Code generation

| Project | Location | Description |
|---------|----------|-------------|
| **Code generator** | [`week4/day3.ipynb`](../week4/day3.ipynb) – [`day5.ipynb`](../week4/day5.ipynb) | LLMs write, explain, and improve Python; compare models side by side. |
| **Supporting modules** | [`week4/styles.py`](../week4/styles.py), [`system_info.py`](../week4/system_info.py) | UI styling and environment helpers for code-gen demos. |

---

### Week 5 — RAG (Retrieval-Augmented Generation)

| Project | Location | Description |
|---------|----------|-------------|
| **RAG week intro** | [`week5/day1.ipynb`](../week5/day1.ipynb) | Embeddings, chunking, and vector stores for **Insurellm** (fictional insurance company). |
| **Knowledge base** | [`week5/knowledge-base/`](../week5/knowledge-base/) | Employee and product markdown docs used as RAG source material. |
| **RAG chatbot** | [`week5/day2.ipynb`](../week5/day2.ipynb) – [`day4.ipynb`](../week5/day4.ipynb) | Build and refine a company Q&A bot with Chroma and LangChain. |
| **Pro RAG implementation** | [`week5/pro_implementation/`](../week5/pro_implementation/) | Production-style ingest + answer pipeline (`ingest.py`, `answer.py`) with advanced chunking. |
| **Advanced RAG** | [`week5/day5.ipynb`](../week5/day5.ipynb) | LLM-driven chunking, preprocessing, and retrieval tuning. |

---

### Week 6 — “The Price is Right” (data & frontier fine-tuning)

| Project | Location | Description |
|---------|----------|-------------|
| **Capstone: price prediction** | [`week6/day1.ipynb`](../week6/day1.ipynb) – [`day5.ipynb`](../week6/day5.ipynb) | Predict Amazon product prices from descriptions: curation, baselines, neural nets, LLMs. |
| **Pricer package** | [`week6/pricer/`](../week6/pricer/) | Data loaders, evaluators, and deep learning models for the pricing task. |
| **Fine-tune frontier model** | [`week6/day5.ipynb`](../week6/day5.ipynb) | OpenAI fine-tuning on product/price pairs. |
| **Optional: deep NN** | [`week6/redemption_train.ipynb`](../week6/redemption_train.ipynb), [`redemption_run.ipynb`](../week6/redemption_run.ipynb) | Train/run a deeper network on the pricing dataset. |

---

### Week 7 — Open-source fine-tuning

| Project | Location | Description |
|---------|----------|-------------|
| **Capstone (continued)** | [`week7/day1.ipynb`](../week7/day1.ipynb), [`day2.ipynb`](../week7/day2.ipynb) | Extend price prediction with open-source evaluation. |
| **Fine-tune Llama (Colab)** | [`week7/day3 and 4.ipynb`](../week7/day3%20and%204.ipynb) | QLoRA fine-tuning on an open-source model for price prediction (Colab link in notebook). |
| **Results & comparison** | [`week7/day5.ipynb`](../week7/day5.ipynb), [`results.ipynb`](../week7/results.ipynb) | Evaluate fine-tuned vs baseline models. |

---

### Week 8 — Agentic AI: deal-hunting system

| Project | Location | Description |
|---------|----------|-------------|
| **The Price is Right (agents)** | [`week8/day1.ipynb`](../week8/day1.ipynb) – [`day5.ipynb`](../week8/day5.ipynb) | Multi-agent framework that scans deals, estimates prices, and surfaces bargains. |
| **Deal agent framework** | [`week8/deal_agent_framework.py`](../week8/deal_agent_framework.py) | Orchestrates planning, scanning, and ensemble pricing agents. |
| **Gradio UI** | [`week8/price_is_right.py`](../week8/price_is_right.py) | “The Price is Right” — autonomous deal-hunting web UI. |
| **Agents** | [`week8/agents/`](../week8/agents/) | Scanner, frontier, specialist, ensemble, planning, and messaging agents. |
| **Modal services** | [`week8/pricer_service.py`](../week8/pricer_service.py), [`pricer_ephemeral.py`](../week8/pricer_ephemeral.py) | Hosted Llama pricing services on Modal. |

---

## Personal projects (`my_llm_projects/`)

| Notebook | Description |
|----------|-------------|
| [`basic_chatbot_llm_wrapper.ipynb`](basic_chatbot_llm_wrapper.ipynb) | CLI chatbot with rolling history, token trimming, and save-on-quit (`chat_history.json`). |
| [`resume_summarizer.ipynb`](resume_summarizer.ipynb) | PDF resume → structured summary (skills, experience, education) via Chat Completions. |

---

## Setup

1. **Python 3.10+** and a virtual environment (the course uses conda/`llms` — see [`setup/SETUP-new.md`](../setup/SETUP-new.md)).

2. **Install dependencies** (full course):

   ```bash
   pip install -r requirements.txt
   ```

   For personal notebooks only:

   ```bash
   pip install openai python-dotenv pypdf ipython
   ```

3. **Environment variables** — create a `.env` at the repo root:

   ```env
   OPENAI_API_KEY=your_key_here
   ```

   Later weeks may also need keys for Anthropic, Google, Groq, HuggingFace, Modal, etc. See the course [resources page](https://edwarddonner.com/2024/11/13/llm-engineering-resources/).

4. **Week 3 & 7:** Some labs run on **Google Colab** (GPU). Links are in the day notebooks.

5. **Resume summarizer:** Place `resume.pdf` next to the notebook or update `pdf_path` in the notebook.

---

## Running

- Open notebooks in Jupyter, VS Code, or Cursor and run cells in order.
- Week 8 agent stack may require **Modal** setup and running services before the Gradio UI.
- Personal chatbot: type `quit` to save and exit.

---

## Notes

- Do not commit `.env`, API keys, or large model weights.
- Add `chat_history.json`, `week8/memory.json`, and local vector DB folders to `.gitignore` when publishing.
- Course material is based on [LLM Engineering](https://edwarddonner.com/2024/11/13/llm-engineering-resources/) by Edward Donner; personal projects are independent exercises.

## License

Course code follows the upstream repository license. Personal projects — use and adapt as you like.
