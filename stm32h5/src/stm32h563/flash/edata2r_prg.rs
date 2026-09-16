///Register `EDATA2R_PRG` reader
pub type R = crate::R<EDATA2R_PRGrs>;
///Register `EDATA2R_PRG` writer
pub type W = crate::W<EDATA2R_PRGrs>;
///Field `EDATA2_STRT` reader - EDATA2_STRT contains the start sectors of the flash high-cycle data area in Bank 2 There is no hardware effect to those bits. They shall be managed by ST tools in Flasher. ... Note: 111: The eight last sectors of the Bank 2 are reserved for flash high-cycle data.
pub type EDATA2_STRT_R = crate::FieldReader;
///Field `EDATA2_STRT` writer - EDATA2_STRT contains the start sectors of the flash high-cycle data area in Bank 2 There is no hardware effect to those bits. They shall be managed by ST tools in Flasher. ... Note: 111: The eight last sectors of the Bank 2 are reserved for flash high-cycle data.
pub type EDATA2_STRT_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `EDATA2_EN` reader - Bank 2 flash high-cycle data enable
pub type EDATA2_EN_R = crate::BitReader;
///Field `EDATA2_EN` writer - Bank 2 flash high-cycle data enable
pub type EDATA2_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:2 - EDATA2_STRT contains the start sectors of the flash high-cycle data area in Bank 2 There is no hardware effect to those bits. They shall be managed by ST tools in Flasher. ... Note: 111: The eight last sectors of the Bank 2 are reserved for flash high-cycle data.
    #[inline(always)]
    pub fn edata2_strt(&self) -> EDATA2_STRT_R {
        EDATA2_STRT_R::new((self.bits & 7) as u8)
    }
    ///Bit 15 - Bank 2 flash high-cycle data enable
    #[inline(always)]
    pub fn edata2_en(&self) -> EDATA2_EN_R {
        EDATA2_EN_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EDATA2R_PRG")
            .field("edata2_strt", &self.edata2_strt())
            .field("edata2_en", &self.edata2_en())
            .finish()
    }
}
impl W {
    ///Bits 0:2 - EDATA2_STRT contains the start sectors of the flash high-cycle data area in Bank 2 There is no hardware effect to those bits. They shall be managed by ST tools in Flasher. ... Note: 111: The eight last sectors of the Bank 2 are reserved for flash high-cycle data.
    #[inline(always)]
    pub fn edata2_strt(&mut self) -> EDATA2_STRT_W<'_, EDATA2R_PRGrs> {
        EDATA2_STRT_W::new(self, 0)
    }
    ///Bit 15 - Bank 2 flash high-cycle data enable
    #[inline(always)]
    pub fn edata2_en(&mut self) -> EDATA2_EN_W<'_, EDATA2R_PRGrs> {
        EDATA2_EN_W::new(self, 15)
    }
}
/**FLASH data sector configuration Bank 2

You can [`read`](crate::Reg::read) this register and get [`edata2r_prg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`edata2r_prg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H563.html#FLASH:EDATA2R_PRG)*/
pub struct EDATA2R_PRGrs;
impl crate::RegisterSpec for EDATA2R_PRGrs {
    type Ux = u32;
}
///`read()` method returns [`edata2r_prg::R`](R) reader structure
impl crate::Readable for EDATA2R_PRGrs {}
///`write(|w| ..)` method takes [`edata2r_prg::W`](W) writer structure
impl crate::Writable for EDATA2R_PRGrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets EDATA2R_PRG to value 0
impl crate::Resettable for EDATA2R_PRGrs {}
