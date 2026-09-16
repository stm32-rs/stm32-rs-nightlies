///Register `OR` reader
pub type R = crate::R<ORrs>;
///Register `OR` writer
pub type W = crate::W<ORrs>;
///Field `VCOREMEN` reader - V less than sub>CORE less than /sub> monitoring
pub type VCOREMEN_R = crate::BitReader;
///Field `VCOREMEN` writer - V less than sub>CORE less than /sub> monitoring
pub type VCOREMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BSEN` reader - Boundary scan enable
pub type BSEN_R = crate::BitReader;
///Field `BSEN` writer - Boundary scan enable
pub type BSEN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - V less than sub>CORE less than /sub> monitoring
    #[inline(always)]
    pub fn vcoremen(&self) -> VCOREMEN_R {
        VCOREMEN_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Boundary scan enable
    #[inline(always)]
    pub fn bsen(&self) -> BSEN_R {
        BSEN_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OR")
            .field("vcoremen", &self.vcoremen())
            .field("bsen", &self.bsen())
            .finish()
    }
}
impl W {
    ///Bit 0 - V less than sub>CORE less than /sub> monitoring
    #[inline(always)]
    pub fn vcoremen(&mut self) -> VCOREMEN_W<'_, ORrs> {
        VCOREMEN_W::new(self, 0)
    }
    ///Bit 1 - Boundary scan enable
    #[inline(always)]
    pub fn bsen(&mut self) -> BSEN_W<'_, ORrs> {
        BSEN_W::new(self, 1)
    }
}
/**TAMP option register

You can [`read`](crate::Reg::read) this register and get [`or::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`or::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#TAMP:OR)*/
pub struct ORrs;
impl crate::RegisterSpec for ORrs {
    type Ux = u32;
}
///`read()` method returns [`or::R`](R) reader structure
impl crate::Readable for ORrs {}
///`write(|w| ..)` method takes [`or::W`](W) writer structure
impl crate::Writable for ORrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets OR to value 0
impl crate::Resettable for ORrs {}
