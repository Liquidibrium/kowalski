"use client"
import Link from "next/link";
import { useState } from "react";

const AnalyzerInput = () => {
  const [loading, setLoading] = useState(false);
  const [url, setUrl] = useState("");


  return (
    <>
      <section
        id="home"
        className="relative z-10 overflow-hidden bg-white pb-16 pt-[120px] dark:bg-gray-dark md:pb-[120px] md:pt-[150px] xl:pb-[160px] xl:pt-[180px] 2xl:pb-[200px] 2xl:pt-[210px]"
      >
        <div className="container">
          <div className="-mx-4 flex flex-wrap">
            <div className="w-full px-4">
              <div className="mx-auto max-w-[800px] text-center flex flex-row gap-2">
                <input
                  type="text"
                  name="link"
                  placeholder="PR link to analyze"
                  className="border-stroke dark:text-body-color-dark dark:shadow-two w-full rounded-sm border bg-[#f8f8f8] px-6 py-3 text-base text-body-color outline-none transition-all duration-300 focus:border-primary dark:border-transparent dark:bg-[#2C303B] dark:focus:border-primary dark:focus:shadow-none"
                  value={url}
                  onChange={(e) => setUrl(e.target.value)}
                />
                <button
                  className="shadow-submit dark:shadow-submit-dark flex  items-center justify-center rounded-sm bg-primary px-9 py-4 text-base font-medium text-white duration-300 hover:bg-primary/90">
                  {loading ? "Analyzing..." : "Analyze"}
                </button>
              </div>
              </div>
            </div>
          </div>
      </section>
    </>
  );
};

export default AnalyzerInput;
