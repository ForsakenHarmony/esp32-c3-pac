#[doc = "Reader of register APB_CTRL_SYSCLK_CONF"]
pub type R = crate::R<u32, super::APB_CTRL_SYSCLK_CONF>;
#[doc = "Writer for register APB_CTRL_SYSCLK_CONF"]
pub type W = crate::W<u32, super::APB_CTRL_SYSCLK_CONF>;
#[doc = "Register APB_CTRL_SYSCLK_CONF `reset()`'s with value 0"]
impl crate::ResetValue for super::APB_CTRL_SYSCLK_CONF {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `APB_CTRL_RST_TICK_CNT`"]
pub type APB_CTRL_RST_TICK_CNT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `APB_CTRL_RST_TICK_CNT`"]
pub struct APB_CTRL_RST_TICK_CNT_W<'a> {
    w: &'a mut W,
}
impl<'a> APB_CTRL_RST_TICK_CNT_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u32) & 0x01) << 12);
        self.w
    }
}
#[doc = "Reader of field `APB_CTRL_CLK_EN`"]
pub type APB_CTRL_CLK_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `APB_CTRL_CLK_EN`"]
pub struct APB_CTRL_CLK_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> APB_CTRL_CLK_EN_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u32) & 0x01) << 11);
        self.w
    }
}
#[doc = "Reader of field `APB_CTRL_CLK_320M_EN`"]
pub type APB_CTRL_CLK_320M_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `APB_CTRL_CLK_320M_EN`"]
pub struct APB_CTRL_CLK_320M_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> APB_CTRL_CLK_320M_EN_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
        self.w
    }
}
#[doc = "Reader of field `APB_CTRL_PRE_DIV_CNT`"]
pub type APB_CTRL_PRE_DIV_CNT_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `APB_CTRL_PRE_DIV_CNT`"]
pub struct APB_CTRL_PRE_DIV_CNT_W<'a> {
    w: &'a mut W,
}
impl<'a> APB_CTRL_PRE_DIV_CNT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03ff) | ((value as u32) & 0x03ff);
        self.w
    }
}
impl R {
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn apb_ctrl_rst_tick_cnt(&self) -> APB_CTRL_RST_TICK_CNT_R {
        APB_CTRL_RST_TICK_CNT_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn apb_ctrl_clk_en(&self) -> APB_CTRL_CLK_EN_R {
        APB_CTRL_CLK_EN_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn apb_ctrl_clk_320m_en(&self) -> APB_CTRL_CLK_320M_EN_R {
        APB_CTRL_CLK_320M_EN_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bits 0:9"]
    #[inline(always)]
    pub fn apb_ctrl_pre_div_cnt(&self) -> APB_CTRL_PRE_DIV_CNT_R {
        APB_CTRL_PRE_DIV_CNT_R::new((self.bits & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn apb_ctrl_rst_tick_cnt(&mut self) -> APB_CTRL_RST_TICK_CNT_W {
        APB_CTRL_RST_TICK_CNT_W { w: self }
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn apb_ctrl_clk_en(&mut self) -> APB_CTRL_CLK_EN_W {
        APB_CTRL_CLK_EN_W { w: self }
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn apb_ctrl_clk_320m_en(&mut self) -> APB_CTRL_CLK_320M_EN_W {
        APB_CTRL_CLK_320M_EN_W { w: self }
    }
    #[doc = "Bits 0:9"]
    #[inline(always)]
    pub fn apb_ctrl_pre_div_cnt(&mut self) -> APB_CTRL_PRE_DIV_CNT_W {
        APB_CTRL_PRE_DIV_CNT_W { w: self }
    }
}
