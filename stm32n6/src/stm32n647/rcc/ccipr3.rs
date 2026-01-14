///Register `CCIPR3` reader
pub type R = crate::R<CCIPR3rs>;
///Register `CCIPR3` writer
pub type W = crate::W<CCIPR3rs>;
///Field `FDCANSEL` reader - Source selection for the FDCAN kernel clock
pub type FDCANSEL_R = crate::FieldReader;
///Field `FDCANSEL` writer - Source selection for the FDCAN kernel clock
pub type FDCANSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `FMCSEL` reader - Source selection for the FMC kernel clock
pub type FMCSEL_R = crate::FieldReader;
///Field `FMCSEL` writer - Source selection for the FMC kernel clock
pub type FMCSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `DFTSEL` reader - Source selection for the DFT kernel clock
pub type DFTSEL_R = crate::BitReader;
///Field `DFTSEL` writer - Source selection for the DFT kernel clock
pub type DFTSEL_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:1 - Source selection for the FDCAN kernel clock
    #[inline(always)]
    pub fn fdcansel(&self) -> FDCANSEL_R {
        FDCANSEL_R::new((self.bits & 3) as u8)
    }
    ///Bits 4:5 - Source selection for the FMC kernel clock
    #[inline(always)]
    pub fn fmcsel(&self) -> FMCSEL_R {
        FMCSEL_R::new(((self.bits >> 4) & 3) as u8)
    }
    ///Bit 8 - Source selection for the DFT kernel clock
    #[inline(always)]
    pub fn dftsel(&self) -> DFTSEL_R {
        DFTSEL_R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CCIPR3")
            .field("fdcansel", &self.fdcansel())
            .field("fmcsel", &self.fmcsel())
            .field("dftsel", &self.dftsel())
            .finish()
    }
}
impl W {
    ///Bits 0:1 - Source selection for the FDCAN kernel clock
    #[inline(always)]
    pub fn fdcansel(&mut self) -> FDCANSEL_W<'_, CCIPR3rs> {
        FDCANSEL_W::new(self, 0)
    }
    ///Bits 4:5 - Source selection for the FMC kernel clock
    #[inline(always)]
    pub fn fmcsel(&mut self) -> FMCSEL_W<'_, CCIPR3rs> {
        FMCSEL_W::new(self, 4)
    }
    ///Bit 8 - Source selection for the DFT kernel clock
    #[inline(always)]
    pub fn dftsel(&mut self) -> DFTSEL_W<'_, CCIPR3rs> {
        DFTSEL_W::new(self, 8)
    }
}
/**RCC clock configuration for independent peripheral register3

You can [`read`](crate::Reg::read) this register and get [`ccipr3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccipr3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RCC:CCIPR3)*/
pub struct CCIPR3rs;
impl crate::RegisterSpec for CCIPR3rs {
    type Ux = u32;
}
///`read()` method returns [`ccipr3::R`](R) reader structure
impl crate::Readable for CCIPR3rs {}
///`write(|w| ..)` method takes [`ccipr3::W`](W) writer structure
impl crate::Writable for CCIPR3rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CCIPR3 to value 0x01
impl crate::Resettable for CCIPR3rs {
    const RESET_VALUE: u32 = 0x01;
}
