#[doc = "Register `EDATA1R_PRG` reader"]
pub type R = crate::R<EDATA1R_PRGrs>;
#[doc = "Register `EDATA1R_PRG` writer"]
pub type W = crate::W<EDATA1R_PRGrs>;
#[doc = "Field `EDATA1_STRT` reader - EDATA1_STRT contains the start sectors of the flash high-cycle data area in Bank 1 There is no hardware effect to those bits. They shall be managed by ST tools in Flasher. ... Note: 111: The eight last sectors of the Bank 1 are reserved for flash high-cycle data"]
pub type EDATA1_STRT_R = crate::FieldReader;
#[doc = "Field `EDATA1_STRT` writer - EDATA1_STRT contains the start sectors of the flash high-cycle data area in Bank 1 There is no hardware effect to those bits. They shall be managed by ST tools in Flasher. ... Note: 111: The eight last sectors of the Bank 1 are reserved for flash high-cycle data"]
pub type EDATA1_STRT_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `EDATA1_EN` reader - Bank 1 flash high-cycle data enable"]
pub type EDATA1_EN_R = crate::BitReader;
#[doc = "Field `EDATA1_EN` writer - Bank 1 flash high-cycle data enable"]
pub type EDATA1_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:2 - EDATA1_STRT contains the start sectors of the flash high-cycle data area in Bank 1 There is no hardware effect to those bits. They shall be managed by ST tools in Flasher. ... Note: 111: The eight last sectors of the Bank 1 are reserved for flash high-cycle data"]
    #[inline(always)]
    pub fn edata1_strt(&self) -> EDATA1_STRT_R {
        EDATA1_STRT_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 15 - Bank 1 flash high-cycle data enable"]
    #[inline(always)]
    pub fn edata1_en(&self) -> EDATA1_EN_R {
        EDATA1_EN_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - EDATA1_STRT contains the start sectors of the flash high-cycle data area in Bank 1 There is no hardware effect to those bits. They shall be managed by ST tools in Flasher. ... Note: 111: The eight last sectors of the Bank 1 are reserved for flash high-cycle data"]
    #[inline(always)]
    #[must_use]
    pub fn edata1_strt(&mut self) -> EDATA1_STRT_W<EDATA1R_PRGrs> {
        EDATA1_STRT_W::new(self, 0)
    }
    #[doc = "Bit 15 - Bank 1 flash high-cycle data enable"]
    #[inline(always)]
    #[must_use]
    pub fn edata1_en(&mut self) -> EDATA1_EN_W<EDATA1R_PRGrs> {
        EDATA1_EN_W::new(self, 15)
    }
}
#[doc = "FLASH data sector configuration Bank 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`edata1r_prg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`edata1r_prg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EDATA1R_PRGrs;
impl crate::RegisterSpec for EDATA1R_PRGrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`edata1r_prg::R`](R) reader structure"]
impl crate::Readable for EDATA1R_PRGrs {}
#[doc = "`write(|w| ..)` method takes [`edata1r_prg::W`](W) writer structure"]
impl crate::Writable for EDATA1R_PRGrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets EDATA1R_PRG to value 0"]
impl crate::Resettable for EDATA1R_PRGrs {
    const RESET_VALUE: u32 = 0;
}
