import React from "react";

type ModalProps = {
  message: string;
  onClose: () => void;
};

const Modal: React.FC<ModalProps> = ({ message, onClose }) => {
  return (
    <div className="fixed inset-0 bg-black/50 flex items-center justify-center z-50">
      <div className="bg-fate-light text-fate-dark font-fate p-6 rounded-2xl shadow-xl w-[90%] max-w-md text-center">
        <p className="text-xl mb-4">{message}</p>
        <button
          className="mt-2 px-4 py-2 bg-fate-accent text-fate-dark rounded hover:bg-fate-softblue transition"
          onClick={onClose}
        >
          わかった、ありがとう
        </button>
      </div>
    </div>
  );
};

export default Modal;
