import ReactMarkdown from "react-markdown";
import remarkGfm from "remark-gfm";

import documentation from "~/assets/doc.md?raw";

export default function Documentation() {
    return (
        <div className="scroll-orange prose prose-invert max-h-full overflow-auto bg-black prose-pre:bg-black p-4 prose-code:text-white prose-headings:text-orange-400 text-white">
            <ReactMarkdown
                remarkPlugins={[remarkGfm]}
                components={{
                    h1: ({ node, ...props }) => (
                        <h1 className="mt-6 mb-2 font-bold text-3xl text-orange-500" {...props} />
                    ),
                    h2: ({ node, ...props }) => (
                        <h2 className="mt-5 mb-2 font-semibold text-2xl text-orange-400" {...props} />
                    ),
                    h3: ({ node, ...props }) => (
                        <h3 className="mt-4 mb-2 font-medium text-orange-300 text-xl" {...props} />
                    ),
                    p: ({ node, ...props }) => <p className="mb-2" {...props} />,
                    pre: ({ node, ...props }) => (
                        <pre className="overflow-x-auto rounded-lg bg-gray-900 p-2 text-sm" {...props} />
                    ),
                    code: ({ node, ...props }) => <code className="rounded bg-gray-800 px-1 text-sm" {...props} />,
                    ul: ({ node, ...props }) => <ul className="mb-2 list-disc pl-6" {...props} />,
                    ol: ({ node, ...props }) => <ol className="mb-2 list-decimal pl-6" {...props} />,
                    li: ({ node, ...props }) => <li className="mb-1" {...props} />,
                    table: ({ node, ...props }) => (
                        <table className="my-4 table-auto border-collapse border border-orange-400" {...props} />
                    ),
                    thead: ({ node, ...props }) => <thead className="bg-orange-900 text-orange-300" {...props} />,
                    tbody: ({ node, ...props }) => <tbody {...props} />,
                    tr: ({ node, ...props }) => <tr className="border-orange-400 border-b" {...props} />,
                    th: ({ node, ...props }) => <th className="border border-orange-400 px-2 py-1" {...props} />,
                    td: ({ node, ...props }) => <td className="border border-orange-400 px-2 py-1" {...props} />,
                }}
            >
                {documentation}
            </ReactMarkdown>
        </div>
    );
}
