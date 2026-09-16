///Register `EDATA1R_PRG` reader
pub type R = crate::R<EDATA1R_PRGrs>;
///Register `EDATA1R_PRG` writer
pub type W = crate::W<EDATA1R_PRGrs>;
///Field `EDATA1_STRT` reader - EDATA1_STRT contains the start sectors of the flash high-cycle data area in Bank 1 There is no hardware effect to those bits. They shall be managed by ST tools in Flasher. ... Note: 111: The eight last sectors of the Bank 1 are reserved for flash high-cycle data
pub type EDATA1_STRT_R = crate::FieldReader;
///Field `EDATA1_STRT` writer - EDATA1_STRT contains the start sectors of the flash high-cycle data area in Bank 1 There is no hardware effect to those bits. They shall be managed by ST tools in Flasher. ... Note: 111: The eight last sectors of the Bank 1 are reserved for flash high-cycle data
pub type EDATA1_STRT_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `EDATA1_EN` reader - Bank 1 flash high-cycle data enable
pub type EDATA1_EN_R = crate::BitReader;
///Field `EDATA1_EN` writer - Bank 1 flash high-cycle data enable
pub type EDATA1_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:2 - EDATA1_STRT contains the start sectors of the flash high-cycle data area in Bank 1 There is no hardware effect to those bits. They shall be managed by ST tools in Flasher. ... Note: 111: The eight last sectors of the Bank 1 are reserved for flash high-cycle data
    #[inline(always)]
    pub fn edata1_strt(&self) -> EDATA1_STRT_R {
        EDATA1_STRT_R::new((self.bits & 7) as u8)
    }
    ///Bit 15 - Bank 1 flash high-cycle data enable
    #[inline(always)]
    pub fn edata1_en(&self) -> EDATA1_EN_R {
        EDATA1_EN_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EDATA1R_PRG")
            .field("edata1_strt", &self.edata1_strt())
            .field("edata1_en", &self.edata1_en())
            .finish()
    }
}
impl W {
    ///Bits 0:2 - EDATA1_STRT contains the start sectors of the flash high-cycle data area in Bank 1 There is no hardware effect to those bits. They shall be managed by ST tools in Flasher. ... Note: 111: The eight last sectors of the Bank 1 are reserved for flash high-cycle data
    #[inline(always)]
    pub fn edata1_strt(&mut self) -> EDATA1_STRT_W<'_, EDATA1R_PRGrs> {
        EDATA1_STRT_W::new(self, 0)
    }
    ///Bit 15 - Bank 1 flash high-cycle data enable
    #[inline(always)]
    pub fn edata1_en(&mut self) -> EDATA1_EN_W<'_, EDATA1R_PRGrs> {
        EDATA1_EN_W::new(self, 15)
    }
}
/**FLASH data sector configuration Bank 1

You can [`read`](crate::Reg::read) this register and get [`edata1r_prg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`edata1r_prg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H573.html#FLASH:EDATA1R_PRG)*/
pub struct EDATA1R_PRGrs;
impl crate::RegisterSpec for EDATA1R_PRGrs {
    type Ux = u32;
}
///`read()` method returns [`edata1r_prg::R`](R) reader structure
impl crate::Readable for EDATA1R_PRGrs {}
///`write(|w| ..)` method takes [`edata1r_prg::W`](W) writer structure
impl crate::Writable for EDATA1R_PRGrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets EDATA1R_PRG to value 0
impl crate::Resettable for EDATA1R_PRGrs {}
