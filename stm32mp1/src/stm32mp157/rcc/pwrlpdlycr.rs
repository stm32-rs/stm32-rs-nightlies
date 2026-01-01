///Register `PWRLPDLYCR` reader
pub type R = crate::R<PWRLPDLYCRrs>;
///Register `PWRLPDLYCR` writer
pub type W = crate::W<PWRLPDLYCRrs>;
///Field `PWRLP_DLY` reader - PWRLP_DLY
pub type PWRLP_DLY_R = crate::FieldReader<u32>;
///Field `PWRLP_DLY` writer - PWRLP_DLY
pub type PWRLP_DLY_W<'a, REG> = crate::FieldWriter<'a, REG, 22, u32>;
///Field `MCTMPSKP` reader - MCTMPSKP
pub type MCTMPSKP_R = crate::BitReader;
///Field `MCTMPSKP` writer - MCTMPSKP
pub type MCTMPSKP_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:21 - PWRLP_DLY
    #[inline(always)]
    pub fn pwrlp_dly(&self) -> PWRLP_DLY_R {
        PWRLP_DLY_R::new(self.bits & 0x003f_ffff)
    }
    ///Bit 24 - MCTMPSKP
    #[inline(always)]
    pub fn mctmpskp(&self) -> MCTMPSKP_R {
        MCTMPSKP_R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PWRLPDLYCR")
            .field("pwrlp_dly", &self.pwrlp_dly())
            .field("mctmpskp", &self.mctmpskp())
            .finish()
    }
}
impl W {
    ///Bits 0:21 - PWRLP_DLY
    #[inline(always)]
    pub fn pwrlp_dly(&mut self) -> PWRLP_DLY_W<'_, PWRLPDLYCRrs> {
        PWRLP_DLY_W::new(self, 0)
    }
    ///Bit 24 - MCTMPSKP
    #[inline(always)]
    pub fn mctmpskp(&mut self) -> MCTMPSKP_W<'_, PWRLPDLYCRrs> {
        MCTMPSKP_W::new(self, 24)
    }
}
/**This register is used to program the delay between the moment where the system exits from one of the Stop modes, and the moment where it is allowed to enable the PLLs and provide a clock to bridges and processors. If TZEN = , this register can only be modified in secure mode.

You can [`read`](crate::Reg::read) this register and get [`pwrlpdlycr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwrlpdlycr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#RCC:PWRLPDLYCR)*/
pub struct PWRLPDLYCRrs;
impl crate::RegisterSpec for PWRLPDLYCRrs {
    type Ux = u32;
}
///`read()` method returns [`pwrlpdlycr::R`](R) reader structure
impl crate::Readable for PWRLPDLYCRrs {}
///`write(|w| ..)` method takes [`pwrlpdlycr::W`](W) writer structure
impl crate::Writable for PWRLPDLYCRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets PWRLPDLYCR to value 0
impl crate::Resettable for PWRLPDLYCRrs {}
