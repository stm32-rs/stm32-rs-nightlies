///Register `OR` reader
pub type R = crate::R<ORrs>;
///Register `OR` writer
pub type W = crate::W<ORrs>;
///Field `SELREF` reader - Internal reference voltage selection
pub type SELREF_R = crate::BitReader;
///Field `SELREF` writer - Internal reference voltage selection
pub type SELREF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SELBG` reader - Bandgap selection
pub type SELBG_R = crate::BitReader;
///Field `SELBG` writer - Bandgap selection
pub type SELBG_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `VDDCOREEN` reader - VDDCORE enable
pub type VDDCOREEN_R = crate::BitReader;
///Field `VDDCOREEN` writer - VDDCORE enable
pub type VDDCOREEN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Internal reference voltage selection
    #[inline(always)]
    pub fn selref(&self) -> SELREF_R {
        SELREF_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Bandgap selection
    #[inline(always)]
    pub fn selbg(&self) -> SELBG_R {
        SELBG_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - VDDCORE enable
    #[inline(always)]
    pub fn vddcoreen(&self) -> VDDCOREEN_R {
        VDDCOREEN_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OR")
            .field("selref", &self.selref())
            .field("selbg", &self.selbg())
            .field("vddcoreen", &self.vddcoreen())
            .finish()
    }
}
impl W {
    ///Bit 0 - Internal reference voltage selection
    #[inline(always)]
    pub fn selref(&mut self) -> SELREF_W<'_, ORrs> {
        SELREF_W::new(self, 0)
    }
    ///Bit 1 - Bandgap selection
    #[inline(always)]
    pub fn selbg(&mut self) -> SELBG_W<'_, ORrs> {
        SELBG_W::new(self, 1)
    }
    ///Bit 2 - VDDCORE enable
    #[inline(always)]
    pub fn vddcoreen(&mut self) -> VDDCOREEN_W<'_, ORrs> {
        VDDCOREEN_W::new(self, 2)
    }
}
/**ADC option register

You can [`read`](crate::Reg::read) this register and get [`or::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`or::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#ADC1:OR)*/
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
