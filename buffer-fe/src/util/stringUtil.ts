export const parseDate = (dateString: string) => {
  let date = new Date(dateString);
  return date.toDateString();
};
