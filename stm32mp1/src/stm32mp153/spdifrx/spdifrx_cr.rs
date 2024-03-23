#[doc = "Register `SPDIFRX_CR` reader"]
pub type R = crate::R<SPDIFRX_CRrs>;
#[doc = "Register `SPDIFRX_CR` writer"]
pub type W = crate::W<SPDIFRX_CRrs>;
#[doc = "Field `SPDIFRXEN` reader - SPDIFRXEN"]
pub type SPDIFRXEN_R = crate::FieldReader;
#[doc = "Field `SPDIFRXEN` writer - SPDIFRXEN"]
pub type SPDIFRXEN_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `RXDMAEN` reader - RXDMAEN"]
pub type RXDMAEN_R = crate::BitReader;
#[doc = "Field `RXDMAEN` writer - RXDMAEN"]
pub type RXDMAEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXSTEO` reader - RXSTEO"]
pub type RXSTEO_R = crate::BitReader;
#[doc = "Field `RXSTEO` writer - RXSTEO"]
pub type RXSTEO_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DRFMT` reader - DRFMT"]
pub type DRFMT_R = crate::FieldReader;
#[doc = "Field `DRFMT` writer - DRFMT"]
pub type DRFMT_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PMSK` reader - PMSK"]
pub type PMSK_R = crate::BitReader;
#[doc = "Field `PMSK` writer - PMSK"]
pub type PMSK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VMSK` reader - VMSK"]
pub type VMSK_R = crate::BitReader;
#[doc = "Field `VMSK` writer - VMSK"]
pub type VMSK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CUMSK` reader - CUMSK"]
pub type CUMSK_R = crate::BitReader;
#[doc = "Field `CUMSK` writer - CUMSK"]
pub type CUMSK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PTMSK` reader - PTMSK"]
pub type PTMSK_R = crate::BitReader;
#[doc = "Field `PTMSK` writer - PTMSK"]
pub type PTMSK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CBDMAEN` reader - CBDMAEN"]
pub type CBDMAEN_R = crate::BitReader;
#[doc = "Field `CBDMAEN` writer - CBDMAEN"]
pub type CBDMAEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CHSEL` reader - CHSEL"]
pub type CHSEL_R = crate::BitReader;
#[doc = "Field `CHSEL` writer - CHSEL"]
pub type CHSEL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NBTR` reader - NBTR"]
pub type NBTR_R = crate::FieldReader;
#[doc = "Field `NBTR` writer - NBTR"]
pub type NBTR_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `WFA` reader - WFA"]
pub type WFA_R = crate::BitReader;
#[doc = "Field `WFA` writer - WFA"]
pub type WFA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INSEL` reader - INSEL"]
pub type INSEL_R = crate::FieldReader;
#[doc = "Field `INSEL` writer - INSEL"]
pub type INSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `CKSEN` reader - CKSEN"]
pub type CKSEN_R = crate::BitReader;
#[doc = "Field `CKSEN` writer - CKSEN"]
pub type CKSEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CKSBKPEN` reader - CKSBKPEN"]
pub type CKSBKPEN_R = crate::BitReader;
#[doc = "Field `CKSBKPEN` writer - CKSBKPEN"]
pub type CKSBKPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1 - SPDIFRXEN"]
    #[inline(always)]
    pub fn spdifrxen(&self) -> SPDIFRXEN_R {
        SPDIFRXEN_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - RXDMAEN"]
    #[inline(always)]
    pub fn rxdmaen(&self) -> RXDMAEN_R {
        RXDMAEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - RXSTEO"]
    #[inline(always)]
    pub fn rxsteo(&self) -> RXSTEO_R {
        RXSTEO_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:5 - DRFMT"]
    #[inline(always)]
    pub fn drfmt(&self) -> DRFMT_R {
        DRFMT_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 6 - PMSK"]
    #[inline(always)]
    pub fn pmsk(&self) -> PMSK_R {
        PMSK_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - VMSK"]
    #[inline(always)]
    pub fn vmsk(&self) -> VMSK_R {
        VMSK_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - CUMSK"]
    #[inline(always)]
    pub fn cumsk(&self) -> CUMSK_R {
        CUMSK_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - PTMSK"]
    #[inline(always)]
    pub fn ptmsk(&self) -> PTMSK_R {
        PTMSK_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - CBDMAEN"]
    #[inline(always)]
    pub fn cbdmaen(&self) -> CBDMAEN_R {
        CBDMAEN_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - CHSEL"]
    #[inline(always)]
    pub fn chsel(&self) -> CHSEL_R {
        CHSEL_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:13 - NBTR"]
    #[inline(always)]
    pub fn nbtr(&self) -> NBTR_R {
        NBTR_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bit 14 - WFA"]
    #[inline(always)]
    pub fn wfa(&self) -> WFA_R {
        WFA_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bits 16:18 - INSEL"]
    #[inline(always)]
    pub fn insel(&self) -> INSEL_R {
        INSEL_R::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bit 20 - CKSEN"]
    #[inline(always)]
    pub fn cksen(&self) -> CKSEN_R {
        CKSEN_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - CKSBKPEN"]
    #[inline(always)]
    pub fn cksbkpen(&self) -> CKSBKPEN_R {
        CKSBKPEN_R::new(((self.bits >> 21) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - SPDIFRXEN"]
    #[inline(always)]
    #[must_use]
    pub fn spdifrxen(&mut self) -> SPDIFRXEN_W<SPDIFRX_CRrs> {
        SPDIFRXEN_W::new(self, 0)
    }
    #[doc = "Bit 2 - RXDMAEN"]
    #[inline(always)]
    #[must_use]
    pub fn rxdmaen(&mut self) -> RXDMAEN_W<SPDIFRX_CRrs> {
        RXDMAEN_W::new(self, 2)
    }
    #[doc = "Bit 3 - RXSTEO"]
    #[inline(always)]
    #[must_use]
    pub fn rxsteo(&mut self) -> RXSTEO_W<SPDIFRX_CRrs> {
        RXSTEO_W::new(self, 3)
    }
    #[doc = "Bits 4:5 - DRFMT"]
    #[inline(always)]
    #[must_use]
    pub fn drfmt(&mut self) -> DRFMT_W<SPDIFRX_CRrs> {
        DRFMT_W::new(self, 4)
    }
    #[doc = "Bit 6 - PMSK"]
    #[inline(always)]
    #[must_use]
    pub fn pmsk(&mut self) -> PMSK_W<SPDIFRX_CRrs> {
        PMSK_W::new(self, 6)
    }
    #[doc = "Bit 7 - VMSK"]
    #[inline(always)]
    #[must_use]
    pub fn vmsk(&mut self) -> VMSK_W<SPDIFRX_CRrs> {
        VMSK_W::new(self, 7)
    }
    #[doc = "Bit 8 - CUMSK"]
    #[inline(always)]
    #[must_use]
    pub fn cumsk(&mut self) -> CUMSK_W<SPDIFRX_CRrs> {
        CUMSK_W::new(self, 8)
    }
    #[doc = "Bit 9 - PTMSK"]
    #[inline(always)]
    #[must_use]
    pub fn ptmsk(&mut self) -> PTMSK_W<SPDIFRX_CRrs> {
        PTMSK_W::new(self, 9)
    }
    #[doc = "Bit 10 - CBDMAEN"]
    #[inline(always)]
    #[must_use]
    pub fn cbdmaen(&mut self) -> CBDMAEN_W<SPDIFRX_CRrs> {
        CBDMAEN_W::new(self, 10)
    }
    #[doc = "Bit 11 - CHSEL"]
    #[inline(always)]
    #[must_use]
    pub fn chsel(&mut self) -> CHSEL_W<SPDIFRX_CRrs> {
        CHSEL_W::new(self, 11)
    }
    #[doc = "Bits 12:13 - NBTR"]
    #[inline(always)]
    #[must_use]
    pub fn nbtr(&mut self) -> NBTR_W<SPDIFRX_CRrs> {
        NBTR_W::new(self, 12)
    }
    #[doc = "Bit 14 - WFA"]
    #[inline(always)]
    #[must_use]
    pub fn wfa(&mut self) -> WFA_W<SPDIFRX_CRrs> {
        WFA_W::new(self, 14)
    }
    #[doc = "Bits 16:18 - INSEL"]
    #[inline(always)]
    #[must_use]
    pub fn insel(&mut self) -> INSEL_W<SPDIFRX_CRrs> {
        INSEL_W::new(self, 16)
    }
    #[doc = "Bit 20 - CKSEN"]
    #[inline(always)]
    #[must_use]
    pub fn cksen(&mut self) -> CKSEN_W<SPDIFRX_CRrs> {
        CKSEN_W::new(self, 20)
    }
    #[doc = "Bit 21 - CKSBKPEN"]
    #[inline(always)]
    #[must_use]
    pub fn cksbkpen(&mut self) -> CKSBKPEN_W<SPDIFRX_CRrs> {
        CKSBKPEN_W::new(self, 21)
    }
}
#[doc = "Control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spdifrx_cr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spdifrx_cr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SPDIFRX_CRrs;
impl crate::RegisterSpec for SPDIFRX_CRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spdifrx_cr::R`](R) reader structure"]
impl crate::Readable for SPDIFRX_CRrs {}
#[doc = "`write(|w| ..)` method takes [`spdifrx_cr::W`](W) writer structure"]
impl crate::Writable for SPDIFRX_CRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SPDIFRX_CR to value 0"]
impl crate::Resettable for SPDIFRX_CRrs {
    const RESET_VALUE: u32 = 0;
}
