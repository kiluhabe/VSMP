FROM balenalib/rpi-python:3.7

WORKDIR /usr/src/app

COPY ./requirements.txt .

RUN apt-get update && apt-get install gcc python-dev zlib1g-dev -y

RUN pip install -r requirements.txt

COPY . ./

# Enable udevd so that plugged dynamic hardware devices show up in our container.
ENV UDEV=1

# main.py will run when container starts up on the device
CMD ["epd7in5"]
