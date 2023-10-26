import { useEffect, useState } from "react";

const CircularSvg = () => (
  <svg className="circular-progress-svg" viewBox="22 22 44 44">
    <circle
      className="CircularProgress-circle"
      cx="44"
      cy="44"
      r="20.2"
      fill="none"
      strokeWidth="3.6"
    ></circle>
  </svg>
);

const circumference = 126.92;
const LoadingCircularSvg = ({
  loading,
  value = 0,
}: {
  loading: boolean;
  value?: number;
}) => {
  const dashoffset = `-${((value / 100) * circumference).toFixed(3)}px`;
  return (
    <svg
      className={
        loading
          ? "circular-progress-loading-svg"
          : "circular-progress-loading-add-svg"
      }
      viewBox="22 22 44 44"
    >
      <circle
        cx="44"
        cy="44"
        r="20.2"
        fill="none"
        strokeWidth="3.6"
        style={{
          strokeDasharray: `${circumference}px`,
          strokeDashoffset: loading ? "80px" : dashoffset,
        }}
      ></circle>
    </svg>
  );
};

const CircularProgress = ({
  loading,
  updatedAt = 0,
}: {
  loading: boolean;
  updatedAt?: number;
}) => {
  const timeToUpdate = 60000;
  const getProgressValue = (updatedAt: number, timeToUpdate: number) =>
    updatedAt
      ? Math.min(100, ((Date.now() - updatedAt) / timeToUpdate) * 100)
      : 0;

  const [value, setValue] = useState(() =>
    getProgressValue(updatedAt, timeToUpdate)
  );

  useEffect(() => {
    setValue(getProgressValue(updatedAt, timeToUpdate));
    const id = setInterval(() => {
      const time = getProgressValue(updatedAt, timeToUpdate);
      setValue(time);
      if (time >= 100) {
        clearInterval(id);
      }
    }, 1000);
    return () => clearInterval(id);
  }, [timeToUpdate, updatedAt]);

  useEffect(() => {
    if (loading) {
      setValue(0);
    }
  }, [loading]);
  return (
    <div className="circular-progress">
      <CircularSvg />
      <LoadingCircularSvg loading={loading} value={value} />
    </div>
  );
};

export default CircularProgress;
