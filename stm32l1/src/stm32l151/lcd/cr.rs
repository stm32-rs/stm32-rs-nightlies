///Register `CR` reader
pub type R = crate::R<CRrs>;
///Register `CR` writer
pub type W = crate::W<CRrs>;
///Field `LCDEN` reader - LCD controller enable
pub type LCDEN_R = crate::BitReader;
///Field `LCDEN` writer - LCD controller enable
pub type LCDEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `VSEL` reader - Voltage source selection
pub type VSEL_R = crate::BitReader;
///Field `VSEL` writer - Voltage source selection
pub type VSEL_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DUTY` reader - Duty selection
pub type DUTY_R = crate::FieldReader;
///Field `DUTY` writer - Duty selection
pub type DUTY_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `BIAS` reader - Bias selector
pub type BIAS_R = crate::FieldReader;
///Field `BIAS` writer - Bias selector
pub type BIAS_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `MUX_SEG` reader - Mux segment enable
pub type MUX_SEG_R = crate::BitReader;
///Field `MUX_SEG` writer - Mux segment enable
pub type MUX_SEG_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - LCD controller enable
    #[inline(always)]
    pub fn lcden(&self) -> LCDEN_R {
        LCDEN_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Voltage source selection
    #[inline(always)]
    pub fn vsel(&self) -> VSEL_R {
        VSEL_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bits 2:4 - Duty selection
    #[inline(always)]
    pub fn duty(&self) -> DUTY_R {
        DUTY_R::new(((self.bits >> 2) & 7) as u8)
    }
    ///Bits 5:6 - Bias selector
    #[inline(always)]
    pub fn bias(&self) -> BIAS_R {
        BIAS_R::new(((self.bits >> 5) & 3) as u8)
    }
    ///Bit 7 - Mux segment enable
    #[inline(always)]
    pub fn mux_seg(&self) -> MUX_SEG_R {
        MUX_SEG_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CR")
            .field("mux_seg", &self.mux_seg())
            .field("bias", &self.bias())
            .field("duty", &self.duty())
            .field("vsel", &self.vsel())
            .field("lcden", &self.lcden())
            .finish()
    }
}
impl W {
    ///Bit 0 - LCD controller enable
    #[inline(always)]
    pub fn lcden(&mut self) -> LCDEN_W<'_, CRrs> {
        LCDEN_W::new(self, 0)
    }
    ///Bit 1 - Voltage source selection
    #[inline(always)]
    pub fn vsel(&mut self) -> VSEL_W<'_, CRrs> {
        VSEL_W::new(self, 1)
    }
    ///Bits 2:4 - Duty selection
    #[inline(always)]
    pub fn duty(&mut self) -> DUTY_W<'_, CRrs> {
        DUTY_W::new(self, 2)
    }
    ///Bits 5:6 - Bias selector
    #[inline(always)]
    pub fn bias(&mut self) -> BIAS_W<'_, CRrs> {
        BIAS_W::new(self, 5)
    }
    ///Bit 7 - Mux segment enable
    #[inline(always)]
    pub fn mux_seg(&mut self) -> MUX_SEG_W<'_, CRrs> {
        MUX_SEG_W::new(self, 7)
    }
}
/**control register

You can [`read`](crate::Reg::read) this register and get [`cr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L151.html#LCD:CR)*/
pub struct CRrs;
impl crate::RegisterSpec for CRrs {
    type Ux = u32;
}
///`read()` method returns [`cr::R`](R) reader structure
impl crate::Readable for CRrs {}
///`write(|w| ..)` method takes [`cr::W`](W) writer structure
impl crate::Writable for CRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CR to value 0
impl crate::Resettable for CRrs {}
