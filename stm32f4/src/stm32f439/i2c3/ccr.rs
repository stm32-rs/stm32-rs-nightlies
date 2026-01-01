///Register `CCR` reader
pub type R = crate::R<CCRrs>;
///Register `CCR` writer
pub type W = crate::W<CCRrs>;
///Field `CCR` reader - Clock control register in Fast/Standard mode (Master mode)
pub type CCR_R = crate::FieldReader<u16>;
///Field `CCR` writer - Clock control register in Fast/Standard mode (Master mode)
pub type CCR_W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
///Field `DUTY` reader - Fast mode duty cycle
pub type DUTY_R = crate::BitReader;
///Field `DUTY` writer - Fast mode duty cycle
pub type DUTY_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `F_S` reader - I2C master mode selection
pub type F_S_R = crate::BitReader;
///Field `F_S` writer - I2C master mode selection
pub type F_S_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:11 - Clock control register in Fast/Standard mode (Master mode)
    #[inline(always)]
    pub fn ccr(&self) -> CCR_R {
        CCR_R::new((self.bits & 0x0fff) as u16)
    }
    ///Bit 14 - Fast mode duty cycle
    #[inline(always)]
    pub fn duty(&self) -> DUTY_R {
        DUTY_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - I2C master mode selection
    #[inline(always)]
    pub fn f_s(&self) -> F_S_R {
        F_S_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CCR")
            .field("f_s", &self.f_s())
            .field("duty", &self.duty())
            .field("ccr", &self.ccr())
            .finish()
    }
}
impl W {
    ///Bits 0:11 - Clock control register in Fast/Standard mode (Master mode)
    #[inline(always)]
    pub fn ccr(&mut self) -> CCR_W<'_, CCRrs> {
        CCR_W::new(self, 0)
    }
    ///Bit 14 - Fast mode duty cycle
    #[inline(always)]
    pub fn duty(&mut self) -> DUTY_W<'_, CCRrs> {
        DUTY_W::new(self, 14)
    }
    ///Bit 15 - I2C master mode selection
    #[inline(always)]
    pub fn f_s(&mut self) -> F_S_W<'_, CCRrs> {
        F_S_W::new(self, 15)
    }
}
/**Clock control register

You can [`read`](crate::Reg::read) this register and get [`ccr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F439.html#I2C3:CCR)*/
pub struct CCRrs;
impl crate::RegisterSpec for CCRrs {
    type Ux = u32;
}
///`read()` method returns [`ccr::R`](R) reader structure
impl crate::Readable for CCRrs {}
///`write(|w| ..)` method takes [`ccr::W`](W) writer structure
impl crate::Writable for CCRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CCR to value 0
impl crate::Resettable for CCRrs {}
