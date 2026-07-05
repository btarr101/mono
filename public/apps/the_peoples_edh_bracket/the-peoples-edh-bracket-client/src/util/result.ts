export type Ok<T> = {
  ty: 'ok'
  value: T
}

export type Err<E> = {
  ty: 'error'
  error: E
}

export type Result<T, E> = Ok<T> | Err<E>

export const ok = <T>(value: T) =>
  ({
    ty: 'ok',
    value,
  }) satisfies Ok<T>

export const err = <E>(error: E) =>
  ({
    ty: 'error',
    error,
  }) satisfies Err<E>
