package com.carlos.admin.utils.page;

import com.github.pagehelper.Page;
import lombok.Getter;
import lombok.Setter;

import java.util.ArrayList;
import java.util.List;

@Setter
@Getter
public class PageResult<T> {

	/**
	 * 当前页码
	 */
	private int pageNum;
	/**
	 * 每页数量
	 */
	private int pageSize;
	/**
	 * 记录总数
	 */
	private int totalSize;
	/**
	 * 页码总数
	 */
	private int totalPages;
	/**
	 * 分页数据
	 */
	private List<T> content  = new ArrayList<>();

	public PageResult() {
	}

	public PageResult(int pageNum, int pageLimit, long totalSize, List<T> content) {
		this(pageNum, pageLimit, Long.valueOf(totalSize).intValue(), content);
	}
	public PageResult(int pageNum, int pageLimit, int totalSize, List<T> content) {
		this.pageNum = pageNum;
		this.pageSize = pageLimit;
		this.totalSize = totalSize;
		this.content = content;
	}

	public PageResult(int pageNum, int pageLimit, int totalSize,int totalPages, List<T> content) {
		this.pageNum = pageNum;
		this.pageSize = pageLimit;
		this.totalSize = totalSize;
		this.totalPages = totalPages;
		this.content = content;
	}

	public static <T> PageResult<T> parse(List<T> list) {
		if (list instanceof Page<T> pagedList) {
			PageResult<T> result = new PageResult<>();
			result.content = pagedList.getResult();
			result.pageNum = pagedList.getPageNum();
			result.pageSize = pagedList.getPageSize();
			result.totalPages = pagedList.getPages();
			if (pagedList.isCount()) {
				result.totalSize = Long.valueOf(pagedList.getTotal()).intValue();
			} else {
				if (pagedList.getResult() != null) {
					if (pagedList.getPageSize() > pagedList.getResult().size()) {
						result.totalSize = (pagedList.getPageNum() - 1) * pagedList.getPageSize() + pagedList.getResult().size();
					} else {
						result.totalSize = pagedList.getPageNum() * pagedList.getPageSize() + 1;
					}
				}
			}

			return result;
		}

		PageResult<T> result = new PageResult<>();
		result.content = list;
		result.pageNum = 1;
		result.pageSize = list.size();
		result.totalSize = list.size();
		return result;
	}
}
