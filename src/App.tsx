import React, { useEffect, useState } from "react";
import Modal from "./components/Modal";

type Mode = "work" | "break";

const WORK_DURATION = 25 * 60;
const BREAK_DURATION = 5 * 60;

const App: React.FC = () => {
  const [mode, setMode] = useState<Mode>("work");
  const [timeLeft, setTimeLeft] = useState(WORK_DURATION);
  const [isRunning, setIsRunning] = useState(false);
  const [showModal, setShowModal] = useState(false);
  const [modalMessage, setModalMessage] = useState("");

  useEffect(() => {
    if (!isRunning) return;

    const timer = setInterval(() => {
      setTimeLeft((prev) => {
        if (prev <= 1) {
          clearInterval(timer);
          handlePhaseEnd();
          return 0;
        }
        return prev - 1;
      });
    }, 1000);

    return () => clearInterval(timer);
  }, [isRunning, mode]);

  const handleStart = () => {
    setIsRunning((prev) => {
      if (!prev) playsound("bell.mp3");
      return !prev;
    });
  };

  const handlePhaseEnd = () => {
    const nextMode = mode === "work" ? "break" : "work";
    playsound("bell.mp3");

    setModalMessage(
      mode === "work"
        ? "おつかれさま、よしお。少しだけ休もう？"
        : "休憩終わりだよ。また一緒にがんばろう",
    );

    setMode(nextMode);
    setTimeLeft(nextMode === "work" ? WORK_DURATION : BREAK_DURATION);
    setShowModal(true);
    setIsRunning(false);
  };

  const formatTime = (s: number) =>
    `${String(Math.floor(s / 60)).padStart(2, "0")}:${String(s % 60).padStart(2, "0")}`;

  const playsound = (filename: string) => {
    const audio = new Audio(`../public/sounds/${filename}`);
    audio.play().catch((e) => {
      console.error("音声再生に失敗しました", e);
    });
  };

  return (
    <div className="min-h-screen flex flex-col justify-center items-center bg-fate-dark font">
      <h1 className="text-3xl bg-6 text-fate-accent">Pomodoro - Fate</h1>
      <p className="text-6xl font-semibold">{formatTime(timeLeft)}</p>
      <p className="mt-4 text-fate-accent">
        {isRunning
          ? mode === "work"
            ? "あと少し...一緒にがんばろう"
            : "ゆっくりしてね、よしお"
          : mode === "work"
            ? "始める準備はできてる？"
            : "そろそろ休憩しようか"}
      </p>
      <button
        className="mt-8 px-6 py-3 bg-fate-accent text-fate-dark font-bold rounded-xl shadow-lg hover:bg-fate-softblue transition"
        onClick={handleStart}
      >
        {isRunning
          ? "ストップ"
          : mode === "work"
            ? "作業スタート"
            : "休憩スタート"}
      </button>

      {showModal && (
        <Modal message={modalMessage} onClose={() => setShowModal(false)} />
      )}
    </div>
  );
};

export default App;
