///Register `C2EMR1` reader
pub type R = crate::R<C2EMR1rs>;
///Register `C2EMR1` writer
pub type W = crate::W<C2EMR1rs>;
///Field `EM0_15` reader - CPU(m) Wakeup with event generation Mask on Event input
pub type EM0_15_R = crate::FieldReader<u16>;
///Field `EM0_15` writer - CPU(m) Wakeup with event generation Mask on Event input
pub type EM0_15_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
///Field `EM17_21` reader - CPU(m) Wakeup with event generation Mask on Event input
pub type EM17_21_R = crate::FieldReader;
///Field `EM17_21` writer - CPU(m) Wakeup with event generation Mask on Event input
pub type EM17_21_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    ///Bits 0:15 - CPU(m) Wakeup with event generation Mask on Event input
    #[inline(always)]
    pub fn em0_15(&self) -> EM0_15_R {
        EM0_15_R::new((self.bits & 0xffff) as u16)
    }
    ///Bits 17:21 - CPU(m) Wakeup with event generation Mask on Event input
    #[inline(always)]
    pub fn em17_21(&self) -> EM17_21_R {
        EM17_21_R::new(((self.bits >> 17) & 0x1f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("C2EMR1")
            .field("em0_15", &self.em0_15())
            .field("em17_21", &self.em17_21())
            .finish()
    }
}
impl W {
    ///Bits 0:15 - CPU(m) Wakeup with event generation Mask on Event input
    #[inline(always)]
    pub fn em0_15(&mut self) -> EM0_15_W<'_, C2EMR1rs> {
        EM0_15_W::new(self, 0)
    }
    ///Bits 17:21 - CPU(m) Wakeup with event generation Mask on Event input
    #[inline(always)]
    pub fn em17_21(&mut self) -> EM17_21_W<'_, C2EMR1rs> {
        EM17_21_W::new(self, 17)
    }
}
/**CPUm wakeup with event mask register

You can [`read`](crate::Reg::read) this register and get [`c2emr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c2emr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB55.html#EXTI:C2EMR1)*/
pub struct C2EMR1rs;
impl crate::RegisterSpec for C2EMR1rs {
    type Ux = u32;
}
///`read()` method returns [`c2emr1::R`](R) reader structure
impl crate::Readable for C2EMR1rs {}
///`write(|w| ..)` method takes [`c2emr1::W`](W) writer structure
impl crate::Writable for C2EMR1rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets C2EMR1 to value 0
impl crate::Resettable for C2EMR1rs {}
