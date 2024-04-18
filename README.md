# Usage:

I find this is most useful when I am connecting to a remote LLM service in kubernetes: where I connect
to the llm service (in this case running text-generation-inference) using:
`kubectl port-forward -n llm-inference services/llm-inference 8081:http`

I then run:
`cargo run -- --input "Summarise this text:\n $(cat ~/Desktop/step.txt)"`

If I want to input richer text format than is easy to type into the CLI, or just a longish file.
