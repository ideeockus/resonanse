import { useEffect, useState } from "react";
import { useNavigate, useParams } from "react-router-dom";
import appAPI, { baseURL } from "../../api/service";
import Loader from "../helpers/loader/loader";
import Back from "./../helpers/back";
import { format } from "date-fns";
import { ru } from "date-fns/locale";
import toast from "react-hot-toast";

const Good = () => {
  const { id } = useParams();
  const nav = useNavigate();
  const [goodInfo, setGoodInfo] = useState(null);

  useEffect(() => {
    async function fetch() {
      const response = await appAPI.getEvent(id);
      if (!response) {
        return setGoodInfo("error");
      }
      try {
        const file = await appAPI.getImage(response.picture);
        if (!file) return setGoodInfo({ ...response, picture: null });
        const localUrl = URL.createObjectURL(file);
        setGoodInfo({ ...response, picture: localUrl });
      } catch (e) {
        return setGoodInfo({ ...response, picture: null });
      }
    }
    fetch();
  }, [id]);

  if (goodInfo === null) return <Loader />;
  if (goodInfo === "error")
    return (
      <div style={{ textAlign: "center", marginTop: "30px" }}>
        Данный товар не найден
      </div>
    );

  return (
    <div className="container">
      <Back />
      <img
        src={
          goodInfo.picture ? goodInfo.picture : "/event.jpg"
        }
        className="good_img"
      />
      <div className="information">
        {goodInfo.title && (
          <div className="information_title">{goodInfo.title}</div>
        )}
        {goodInfo.description && (
          <div className="information_description">{goodInfo.description}</div>
        )}
        <div className="information_date">
          📅{" "}
          {format(new Date(goodInfo.datetime_from), "dd MMMM yyyy", {
            locale: ru,
          })}
        </div>
        <div className="information_date">
          ⏰ {format(new Date(goodInfo.datetime_from), "HH:mm")}
        </div>
        <div className="information_date">📍 {goodInfo.location_title}</div>
        {goodInfo.contact && (
          <div className="information_date">
            ☎️{" "}
            <span /*onClick={() => window.open(`https://t.me/${"kulbabus"}`)}*/>
              {goodInfo.contact}
            </span>
          </div>
        )}

        {/*<div className="information_button">
          <img src="/menu.svg" />
          Гео
        </div>*/}
      </div>
    </div>
  );
};

export default Good;
