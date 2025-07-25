///Register `MACPPSTTN1R` reader
pub type R = crate::R<MACPPSTTN1Rrs>;
///Register `MACPPSTTN1R` writer
pub type W = crate::W<MACPPSTTN1Rrs>;
///Field `TTSL0` reader - Target Time Low for PPS Register
pub type TTSL0_R = crate::FieldReader<u32>;
///Field `TTSL0` writer - Target Time Low for PPS Register
pub type TTSL0_W<'a, REG> = crate::FieldWriter<'a, REG, 31, u32>;
///Field `TRGTBUSY0` reader - PPS Target Time Register Busy
pub type TRGTBUSY0_R = crate::BitReader;
///Field `TRGTBUSY0` writer - PPS Target Time Register Busy
pub type TRGTBUSY0_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:30 - Target Time Low for PPS Register
    #[inline(always)]
    pub fn ttsl0(&self) -> TTSL0_R {
        TTSL0_R::new(self.bits & 0x7fff_ffff)
    }
    ///Bit 31 - PPS Target Time Register Busy
    #[inline(always)]
    pub fn trgtbusy0(&self) -> TRGTBUSY0_R {
        TRGTBUSY0_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MACPPSTTN1R")
            .field("ttsl0", &self.ttsl0())
            .field("trgtbusy0", &self.trgtbusy0())
            .finish()
    }
}
impl W {
    ///Bits 0:30 - Target Time Low for PPS Register
    #[inline(always)]
    pub fn ttsl0(&mut self) -> TTSL0_W<MACPPSTTN1Rrs> {
        TTSL0_W::new(self, 0)
    }
    ///Bit 31 - PPS Target Time Register Busy
    #[inline(always)]
    pub fn trgtbusy0(&mut self) -> TRGTBUSY0_W<MACPPSTTN1Rrs> {
        TRGTBUSY0_W::new(self, 31)
    }
}
/**PPS 1 target time nanoseconds register

You can [`read`](crate::Reg::read) this register and get [`macppsttn1r::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`macppsttn1r::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#ETH:MACPPSTTN1R)*/
pub struct MACPPSTTN1Rrs;
impl crate::RegisterSpec for MACPPSTTN1Rrs {
    type Ux = u32;
}
///`read()` method returns [`macppsttn1r::R`](R) reader structure
impl crate::Readable for MACPPSTTN1Rrs {}
///`write(|w| ..)` method takes [`macppsttn1r::W`](W) writer structure
impl crate::Writable for MACPPSTTN1Rrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets MACPPSTTN1R to value 0
impl crate::Resettable for MACPPSTTN1Rrs {}
