///Register `CSR` reader
pub type R = crate::R<CSRrs>;
///Register `CSR` writer
pub type W = crate::W<CSRrs>;
///Field `WUF` reader - Wakeup flag
pub type WUF_R = crate::BitReader;
///Field `SBF` reader - Standby flag
pub type SBF_R = crate::BitReader;
///Field `EWUP1` reader - Enable WKUP pin 1
pub type EWUP1_R = crate::BitReader;
///Field `EWUP1` writer - Enable WKUP pin 1
pub type EWUP1_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EWUP2` reader - Enable WKUP pin 2
pub type EWUP2_R = crate::BitReader;
///Field `EWUP2` writer - Enable WKUP pin 2
pub type EWUP2_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EWUP4` reader - Enable WKUP pin 4
pub type EWUP4_R = crate::BitReader;
///Field `EWUP4` writer - Enable WKUP pin 4
pub type EWUP4_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EWUP5` reader - Enable WKUP pin 5
pub type EWUP5_R = crate::BitReader;
///Field `EWUP5` writer - Enable WKUP pin 5
pub type EWUP5_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EWUP6` reader - Enable WKUP pin 6
pub type EWUP6_R = crate::BitReader;
///Field `EWUP6` writer - Enable WKUP pin 6
pub type EWUP6_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EWUP7` reader - Enable WKUP pin 7
pub type EWUP7_R = crate::BitReader;
///Field `EWUP7` writer - Enable WKUP pin 7
pub type EWUP7_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Wakeup flag
    #[inline(always)]
    pub fn wuf(&self) -> WUF_R {
        WUF_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Standby flag
    #[inline(always)]
    pub fn sbf(&self) -> SBF_R {
        SBF_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 8 - Enable WKUP pin 1
    #[inline(always)]
    pub fn ewup1(&self) -> EWUP1_R {
        EWUP1_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Enable WKUP pin 2
    #[inline(always)]
    pub fn ewup2(&self) -> EWUP2_R {
        EWUP2_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 11 - Enable WKUP pin 4
    #[inline(always)]
    pub fn ewup4(&self) -> EWUP4_R {
        EWUP4_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - Enable WKUP pin 5
    #[inline(always)]
    pub fn ewup5(&self) -> EWUP5_R {
        EWUP5_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - Enable WKUP pin 6
    #[inline(always)]
    pub fn ewup6(&self) -> EWUP6_R {
        EWUP6_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - Enable WKUP pin 7
    #[inline(always)]
    pub fn ewup7(&self) -> EWUP7_R {
        EWUP7_R::new(((self.bits >> 14) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CSR")
            .field("wuf", &self.wuf())
            .field("sbf", &self.sbf())
            .field("ewup1", &self.ewup1())
            .field("ewup2", &self.ewup2())
            .field("ewup4", &self.ewup4())
            .field("ewup5", &self.ewup5())
            .field("ewup6", &self.ewup6())
            .field("ewup7", &self.ewup7())
            .finish()
    }
}
impl W {
    ///Bit 8 - Enable WKUP pin 1
    #[inline(always)]
    pub fn ewup1(&mut self) -> EWUP1_W<'_, CSRrs> {
        EWUP1_W::new(self, 8)
    }
    ///Bit 9 - Enable WKUP pin 2
    #[inline(always)]
    pub fn ewup2(&mut self) -> EWUP2_W<'_, CSRrs> {
        EWUP2_W::new(self, 9)
    }
    ///Bit 11 - Enable WKUP pin 4
    #[inline(always)]
    pub fn ewup4(&mut self) -> EWUP4_W<'_, CSRrs> {
        EWUP4_W::new(self, 11)
    }
    ///Bit 12 - Enable WKUP pin 5
    #[inline(always)]
    pub fn ewup5(&mut self) -> EWUP5_W<'_, CSRrs> {
        EWUP5_W::new(self, 12)
    }
    ///Bit 13 - Enable WKUP pin 6
    #[inline(always)]
    pub fn ewup6(&mut self) -> EWUP6_W<'_, CSRrs> {
        EWUP6_W::new(self, 13)
    }
    ///Bit 14 - Enable WKUP pin 7
    #[inline(always)]
    pub fn ewup7(&mut self) -> EWUP7_W<'_, CSRrs> {
        EWUP7_W::new(self, 14)
    }
}
/**power control/status register

You can [`read`](crate::Reg::read) this register and get [`csr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F0x0.html#PWR:CSR)*/
pub struct CSRrs;
impl crate::RegisterSpec for CSRrs {
    type Ux = u32;
}
///`read()` method returns [`csr::R`](R) reader structure
impl crate::Readable for CSRrs {}
///`write(|w| ..)` method takes [`csr::W`](W) writer structure
impl crate::Writable for CSRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CSR to value 0
impl crate::Resettable for CSRrs {}
