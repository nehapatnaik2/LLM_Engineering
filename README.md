# My LLM Projects

Small experiments and utilities built with the OpenAI API while learning LLM engineering. Each project is a Jupyter notebook you can run locally.

## Projects

| Notebook | Description |
|----------|-------------|
| [`basic_chatbot_llm_wrapper.ipynb`](basic_chatbot_llm_wrapper.ipynb) | Interactive CLI chatbot with persistent history (`chat_history.json`), rolling context window, and save-on-quit. |
| [`resume_summarizer.ipynb`](resume_summarizer.ipynb) | Reads a PDF resume and returns a structured summary (skills, experience, education) via the Chat Completions API. |

## Setup

1. **Python 3.10+** and a virtual environment (recommended).

2. **Install dependencies:**

   ```bash
   pip install openai python-dotenv pypdf ipython
   ```

3. **API key:** Create a `.env` file in this directory:

   ```env
   OPENAI_API_KEY=your_key_here
   ```

4. **Resume summarizer only:** Place your PDF as `resume.pdf` next to the notebook (or update `pdf_path` in the notebook).

## Running

Open a notebook in Jupyter or VS Code/Cursor and run the cells in order. For the chatbot, type `quit` to exit and save the conversation.

Both notebooks use `gpt-4o-mini` by default to keep API costs low.

## Notes

- `chat_history.json` is created at runtime by the chatbot; add it to `.gitignore` if you publish this repo.
- Do not commit `.env` or API keys.

## License

Personal learning projects — use and adapt as you like.
