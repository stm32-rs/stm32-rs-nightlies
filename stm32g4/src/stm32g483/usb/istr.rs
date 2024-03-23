#[doc = "Register `ISTR` reader"]
pub type R = crate::R<ISTRrs>;
#[doc = "Register `ISTR` writer"]
pub type W = crate::W<ISTRrs>;
#[doc = "Field `EP_ID` reader - EP_ID"]
pub type EP_ID_R = crate::FieldReader;
#[doc = "Field `EP_ID` writer - EP_ID"]
pub type EP_ID_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `DIR` reader - DIR"]
pub type DIR_R = crate::BitReader;
#[doc = "Field `DIR` writer - DIR"]
pub type DIR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `L1REQ` reader - L1REQ"]
pub type L1REQ_R = crate::BitReader;
#[doc = "Field `L1REQ` writer - L1REQ"]
pub type L1REQ_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ESOF` reader - ESOF"]
pub type ESOF_R = crate::BitReader;
#[doc = "Field `ESOF` writer - ESOF"]
pub type ESOF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SOF` reader - SOF"]
pub type SOF_R = crate::BitReader;
#[doc = "Field `SOF` writer - SOF"]
pub type SOF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESET` reader - RESET"]
pub type RESET_R = crate::BitReader;
#[doc = "Field `RESET` writer - RESET"]
pub type RESET_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SUSP` reader - SUSP"]
pub type SUSP_R = crate::BitReader;
#[doc = "Field `SUSP` writer - SUSP"]
pub type SUSP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WKUP` reader - WKUP"]
pub type WKUP_R = crate::BitReader;
#[doc = "Field `WKUP` writer - WKUP"]
pub type WKUP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ERR` reader - ERR"]
pub type ERR_R = crate::BitReader;
#[doc = "Field `ERR` writer - ERR"]
pub type ERR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PMAOVR` reader - PMAOVR"]
pub type PMAOVR_R = crate::BitReader;
#[doc = "Field `PMAOVR` writer - PMAOVR"]
pub type PMAOVR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CTR` reader - CTR"]
pub type CTR_R = crate::BitReader;
#[doc = "Field `CTR` writer - CTR"]
pub type CTR_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:3 - EP_ID"]
    #[inline(always)]
    pub fn ep_id(&self) -> EP_ID_R {
        EP_ID_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 4 - DIR"]
    #[inline(always)]
    pub fn dir(&self) -> DIR_R {
        DIR_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 7 - L1REQ"]
    #[inline(always)]
    pub fn l1req(&self) -> L1REQ_R {
        L1REQ_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - ESOF"]
    #[inline(always)]
    pub fn esof(&self) -> ESOF_R {
        ESOF_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - SOF"]
    #[inline(always)]
    pub fn sof(&self) -> SOF_R {
        SOF_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - RESET"]
    #[inline(always)]
    pub fn reset(&self) -> RESET_R {
        RESET_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - SUSP"]
    #[inline(always)]
    pub fn susp(&self) -> SUSP_R {
        SUSP_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - WKUP"]
    #[inline(always)]
    pub fn wkup(&self) -> WKUP_R {
        WKUP_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - ERR"]
    #[inline(always)]
    pub fn err(&self) -> ERR_R {
        ERR_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - PMAOVR"]
    #[inline(always)]
    pub fn pmaovr(&self) -> PMAOVR_R {
        PMAOVR_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - CTR"]
    #[inline(always)]
    pub fn ctr(&self) -> CTR_R {
        CTR_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - EP_ID"]
    #[inline(always)]
    #[must_use]
    pub fn ep_id(&mut self) -> EP_ID_W<ISTRrs> {
        EP_ID_W::new(self, 0)
    }
    #[doc = "Bit 4 - DIR"]
    #[inline(always)]
    #[must_use]
    pub fn dir(&mut self) -> DIR_W<ISTRrs> {
        DIR_W::new(self, 4)
    }
    #[doc = "Bit 7 - L1REQ"]
    #[inline(always)]
    #[must_use]
    pub fn l1req(&mut self) -> L1REQ_W<ISTRrs> {
        L1REQ_W::new(self, 7)
    }
    #[doc = "Bit 8 - ESOF"]
    #[inline(always)]
    #[must_use]
    pub fn esof(&mut self) -> ESOF_W<ISTRrs> {
        ESOF_W::new(self, 8)
    }
    #[doc = "Bit 9 - SOF"]
    #[inline(always)]
    #[must_use]
    pub fn sof(&mut self) -> SOF_W<ISTRrs> {
        SOF_W::new(self, 9)
    }
    #[doc = "Bit 10 - RESET"]
    #[inline(always)]
    #[must_use]
    pub fn reset(&mut self) -> RESET_W<ISTRrs> {
        RESET_W::new(self, 10)
    }
    #[doc = "Bit 11 - SUSP"]
    #[inline(always)]
    #[must_use]
    pub fn susp(&mut self) -> SUSP_W<ISTRrs> {
        SUSP_W::new(self, 11)
    }
    #[doc = "Bit 12 - WKUP"]
    #[inline(always)]
    #[must_use]
    pub fn wkup(&mut self) -> WKUP_W<ISTRrs> {
        WKUP_W::new(self, 12)
    }
    #[doc = "Bit 13 - ERR"]
    #[inline(always)]
    #[must_use]
    pub fn err(&mut self) -> ERR_W<ISTRrs> {
        ERR_W::new(self, 13)
    }
    #[doc = "Bit 14 - PMAOVR"]
    #[inline(always)]
    #[must_use]
    pub fn pmaovr(&mut self) -> PMAOVR_W<ISTRrs> {
        PMAOVR_W::new(self, 14)
    }
    #[doc = "Bit 15 - CTR"]
    #[inline(always)]
    #[must_use]
    pub fn ctr(&mut self) -> CTR_W<ISTRrs> {
        CTR_W::new(self, 15)
    }
}
#[doc = "USB interrupt status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`istr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`istr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ISTRrs;
impl crate::RegisterSpec for ISTRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`istr::R`](R) reader structure"]
impl crate::Readable for ISTRrs {}
#[doc = "`write(|w| ..)` method takes [`istr::W`](W) writer structure"]
impl crate::Writable for ISTRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ISTR to value 0"]
impl crate::Resettable for ISTRrs {
    const RESET_VALUE: u32 = 0;
}
