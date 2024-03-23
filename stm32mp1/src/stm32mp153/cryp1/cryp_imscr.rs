#[doc = "Register `CRYP_IMSCR` reader"]
pub type R = crate::R<CRYP_IMSCRrs>;
#[doc = "Register `CRYP_IMSCR` writer"]
pub type W = crate::W<CRYP_IMSCRrs>;
#[doc = "Field `INIM` reader - INIM"]
pub type INIM_R = crate::BitReader;
#[doc = "Field `INIM` writer - INIM"]
pub type INIM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OUTIM` reader - OUTIM"]
pub type OUTIM_R = crate::BitReader;
#[doc = "Field `OUTIM` writer - OUTIM"]
pub type OUTIM_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - INIM"]
    #[inline(always)]
    pub fn inim(&self) -> INIM_R {
        INIM_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - OUTIM"]
    #[inline(always)]
    pub fn outim(&self) -> OUTIM_R {
        OUTIM_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - INIM"]
    #[inline(always)]
    #[must_use]
    pub fn inim(&mut self) -> INIM_W<CRYP_IMSCRrs> {
        INIM_W::new(self, 0)
    }
    #[doc = "Bit 1 - OUTIM"]
    #[inline(always)]
    #[must_use]
    pub fn outim(&mut self) -> OUTIM_W<CRYP_IMSCRrs> {
        OUTIM_W::new(self, 1)
    }
}
#[doc = "The CRYP_IMSCR register is the interrupt mask set or clear register. It is a read/write register. When a read operation is performed, this register gives the current value of the mask applied to the relevant interrupt. Writing 1 to the particular bit sets the mask, thus enabling the interrupt to be read. Writing 0 to this bit clears the corresponding mask. All the bits are cleared to 0 when the peripheral is reset.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cryp_imscr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cryp_imscr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CRYP_IMSCRrs;
impl crate::RegisterSpec for CRYP_IMSCRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cryp_imscr::R`](R) reader structure"]
impl crate::Readable for CRYP_IMSCRrs {}
#[doc = "`write(|w| ..)` method takes [`cryp_imscr::W`](W) writer structure"]
impl crate::Writable for CRYP_IMSCRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CRYP_IMSCR to value 0"]
impl crate::Resettable for CRYP_IMSCRrs {
    const RESET_VALUE: u32 = 0;
}
