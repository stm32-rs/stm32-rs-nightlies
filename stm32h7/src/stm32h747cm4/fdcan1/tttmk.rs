#[doc = "Register `TTTMK` reader"]
pub type R = crate::R<TTTMKrs>;
#[doc = "Register `TTTMK` writer"]
pub type W = crate::W<TTTMKrs>;
#[doc = "Field `TM` reader - Time Mark"]
pub type TM_R = crate::FieldReader<u16>;
#[doc = "Field `TM` writer - Time Mark"]
pub type TM_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `TICC` reader - Time Mark Cycle Code"]
pub type TICC_R = crate::FieldReader;
#[doc = "Field `TICC` writer - Time Mark Cycle Code"]
pub type TICC_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `LCKM` reader - TT Time Mark Register Locked"]
pub type LCKM_R = crate::BitReader;
#[doc = "Field `LCKM` writer - TT Time Mark Register Locked"]
pub type LCKM_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:15 - Time Mark"]
    #[inline(always)]
    pub fn tm(&self) -> TM_R {
        TM_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:22 - Time Mark Cycle Code"]
    #[inline(always)]
    pub fn ticc(&self) -> TICC_R {
        TICC_R::new(((self.bits >> 16) & 0x7f) as u8)
    }
    #[doc = "Bit 31 - TT Time Mark Register Locked"]
    #[inline(always)]
    pub fn lckm(&self) -> LCKM_R {
        LCKM_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - Time Mark"]
    #[inline(always)]
    #[must_use]
    pub fn tm(&mut self) -> TM_W<TTTMKrs> {
        TM_W::new(self, 0)
    }
    #[doc = "Bits 16:22 - Time Mark Cycle Code"]
    #[inline(always)]
    #[must_use]
    pub fn ticc(&mut self) -> TICC_W<TTTMKrs> {
        TICC_W::new(self, 16)
    }
    #[doc = "Bit 31 - TT Time Mark Register Locked"]
    #[inline(always)]
    #[must_use]
    pub fn lckm(&mut self) -> LCKM_W<TTTMKrs> {
        LCKM_W::new(self, 31)
    }
}
#[doc = "FDCAN TT Time Mark Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tttmk::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tttmk::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TTTMKrs;
impl crate::RegisterSpec for TTTMKrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tttmk::R`](R) reader structure"]
impl crate::Readable for TTTMKrs {}
#[doc = "`write(|w| ..)` method takes [`tttmk::W`](W) writer structure"]
impl crate::Writable for TTTMKrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TTTMK to value 0"]
impl crate::Resettable for TTTMKrs {
    const RESET_VALUE: u32 = 0;
}
