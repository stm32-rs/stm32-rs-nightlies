///Register `OR` reader
pub type R = crate::R<ORrs>;
///Register `OR` writer
pub type W = crate::W<ORrs>;
///Field `OP0` reader - Option bit 0
pub type OP0_R = crate::BitReader;
///Field `OP0` writer - Option bit 0
pub type OP0_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OP1` reader - Option bit 1
pub type OP1_R = crate::BitReader;
///Field `OP1` writer - Option bit 1
pub type OP1_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Option bit 0
    #[inline(always)]
    pub fn op0(&self) -> OP0_R {
        OP0_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Option bit 1
    #[inline(always)]
    pub fn op1(&self) -> OP1_R {
        OP1_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OR")
            .field("op0", &self.op0())
            .field("op1", &self.op1())
            .finish()
    }
}
impl W {
    ///Bit 0 - Option bit 0
    #[inline(always)]
    pub fn op0(&mut self) -> OP0_W<'_, ORrs> {
        OP0_W::new(self, 0)
    }
    ///Bit 1 - Option bit 1
    #[inline(always)]
    pub fn op1(&mut self) -> OP1_W<'_, ORrs> {
        OP1_W::new(self, 1)
    }
}
/**ADC option register

You can [`read`](crate::Reg::read) this register and get [`or::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`or::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H503.html#ADC1:OR)*/
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
