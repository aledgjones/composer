import { FC, HTMLAttributes, DetailedHTMLProps } from "react";

type SuperProps<Element, Props> = Props & DetailedHTMLProps<HTMLAttributes<Element>, Element>;
export type SuperFC<Element, Props> = FC<SuperProps<Element, Props>>;
