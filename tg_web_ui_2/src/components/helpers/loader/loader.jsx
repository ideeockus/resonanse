const Loader = () => {
  return (
    <div className="loader_block">
      <svg
        xmlns="http://www.w3.org/2000/svg"
        width="197px"
        height="197px"
        viewBox="0 0 100 100"
        preserveAspectRatio="xMidYMid"
      >
        <circle
          cx="50"
          cy="50"
          fill="none"
          strokeWidth="3"
          r="15"
          strokeDasharray="70.68583470577033 25.561944901923447"
        >
          <animateTransform
            attributeName="transform"
            type="rotate"
            repeatCount="indefinite"
            dur="0.8695652173913042s"
            values="0 50 50;360 50 50"
            keyTimes="0;1"
          ></animateTransform>
        </circle>
      </svg>
    </div>
  );
};

export default Loader;
