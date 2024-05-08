import { useEffect, useState } from "react";
import { useNavigate } from "react-router-dom";
import appAPI from "../../api/service";
import Loader from "../helpers/loader/loader";
import toast from "react-hot-toast";
import { format } from "date-fns";
import { ru } from "date-fns/locale";

const Main = () => {
  const nav = useNavigate();
  const [search, setSearch] = useState("");
  const [goods, setGoods] = useState(null);
  const [filteredGoods, setFilteredGoods] = useState(null);

  useEffect(() => {
    const fetchEvents = async () => {
      try {
        const response = await appAPI.getEvents();
        if (response) {
          setGoods(response);
        } else {
          toast.error("Ошибка при запросе событий");
        }
      } catch (error) {
        console.error(error);
      }
    };

    fetchEvents();
  }, []);

  const [loadedImages, setLoadedImages] = useState({});

  const handleImageLoad = async (good) => {
    try {
      if (!loadedImages[good.id]) {
        const file = await appAPI.getImage(good.picture);
        if (file) {
          const imageUrl = URL.createObjectURL(file);
          setGoods((prevGoods) => {
            const updatedGoods = prevGoods.map((item) => {
              if (item.id === good.id) {
                return { ...item, poster_image_link: imageUrl };
              }
              return item;
            });
            return updatedGoods;
          });
          setLoadedImages((prevState) => ({
            ...prevState,
            [good.id]: true,
          }));
        } else {
          setGoods((prevGoods) => {
            const updatedGoods = prevGoods.map((item) => {
              if (item.id === good.id) {
                return { ...item, poster_image_link: null };
              }
              return item;
            });
            return updatedGoods;
          });
          setLoadedImages((prevState) => ({
            ...prevState,
            [good.id]: true,
          }));
        }
      }
    } catch (error) {
      console.error(error);
    }
  };


  useEffect(() => {
    if (!goods) return;
    if (search === "") {
      setFilteredGoods(goods);
    } else {
      let timer;
      if (!timer) {
        timer = setTimeout(() => {
          setFilteredGoods(goods.filter((good) => good.title.toUpperCase().includes(search.toUpperCase())));
        }, 1000);
      }
      return () => clearTimeout(timer);
    }
  }, [goods, search]);

  return (
    <div className="container" style={{ paddingTop: "10px" }}>
      <div className="search_block">
        <label className={`input_search`} tabIndex="0">
          <div className={`search_icon`}>
            <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 50 50">
              <path d="M 21 3 C 11.621094 3 4 10.621094 4 20 C 4 29.378906 11.621094 37 21 37 C 24.710938 37 28.140625 35.804688 30.9375 33.78125 L 44.09375 46.90625 L 46.90625 44.09375 L 33.90625 31.0625 C 36.460938 28.085938 38 24.222656 38 20 C 38 10.621094 30.378906 3 21 3 Z M 21 5 C 29.296875 5 36 11.703125 36 20 C 36 28.296875 29.296875 35 21 35 C 12.703125 35 6 28.296875 6 20 C 6 11.703125 12.703125 5 21 5 Z" />
            </svg>
          </div>
          <input
            type="text"
            placeholder="Поиск"
            value={search}
            onChange={(e) => setSearch(e.target.value)}
          />
          {search !== "" && (
            <div
              className="close_icon"
              onClick={(e) => {
                setSearch("");
              }}
            >
              <xml version="1.0" encoding="utf-8" />
              <svg
                viewBox="0 0 24 24"
                fill="none"
                xmlns="http://www.w3.org/2000/svg"
              >
                <rect width="24" height="24" fill="none" />
                <path
                  d="M7 17L16.8995 7.10051"
                  strokeLinecap="round"
                  strokeLinejoin="round"
                />
                <path
                  d="M7 7.00001L16.8995 16.8995"
                  strokeLinecap="round"
                  strokeLinejoin="round"
                />
              </svg>
            </div>
          )}
        </label>
      </div>
      <div className="catalog">
        {filteredGoods === null ? (
          <Loader />
        ) : (
          filteredGoods.map((good) => (
            <div
              className="cart"
              key={good.id}
              onClick={() => nav(`/good/${good.id}`)}
            >
              <img
                src={
                  good.poster_image_link ? good.poster_image_link : "/event.jpg"
                }
                className="image"
                onLoad={() => handleImageLoad(good)}
                alt={good.title}
              />
              <div className="info">
                <div className="title">{good.title}</div>
                <div className="date">
                  📅{" "}
                  {format(new Date(good.datetime_from), "dd MMMM yyyy", {
                    locale: ru,
                  })}
                </div>
                <div className="time">
                  ⏰ {format(new Date(good.datetime_from), "HH:mm")}
                </div>
              </div>
            </div>
          ))
        )}
      </div>
    </div>
  );
};

export default Main;
