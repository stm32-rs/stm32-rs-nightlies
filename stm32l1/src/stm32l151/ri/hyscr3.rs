///Register `HYSCR3` reader
pub type R = crate::R<HYSCR3rs>;
///Register `HYSCR3` writer
pub type W = crate::W<HYSCR3rs>;
///Field `PE` reader - Port E hysteresis control on/off
pub type PE_R = crate::FieldReader<u16>;
///Field `PE` writer - Port E hysteresis control on/off
pub type PE_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
///Field `PF` reader - Port F hysteresis control on/off
pub type PF_R = crate::FieldReader<u16>;
///Field `PF` writer - Port F hysteresis control on/off
pub type PF_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    ///Bits 0:15 - Port E hysteresis control on/off
    #[inline(always)]
    pub fn pe(&self) -> PE_R {
        PE_R::new((self.bits & 0xffff) as u16)
    }
    ///Bits 16:31 - Port F hysteresis control on/off
    #[inline(always)]
    pub fn pf(&self) -> PF_R {
        PF_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HYSCR3")
            .field("pf", &self.pf())
            .field("pe", &self.pe())
            .finish()
    }
}
impl W {
    ///Bits 0:15 - Port E hysteresis control on/off
    #[inline(always)]
    pub fn pe(&mut self) -> PE_W<'_, HYSCR3rs> {
        PE_W::new(self, 0)
    }
    ///Bits 16:31 - Port F hysteresis control on/off
    #[inline(always)]
    pub fn pf(&mut self) -> PF_W<'_, HYSCR3rs> {
        PF_W::new(self, 16)
    }
}
/**RI hysteresis control register 3

You can [`read`](crate::Reg::read) this register and get [`hyscr3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hyscr3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L151.html#RI:HYSCR3)*/
pub struct HYSCR3rs;
impl crate::RegisterSpec for HYSCR3rs {
    type Ux = u32;
}
///`read()` method returns [`hyscr3::R`](R) reader structure
impl crate::Readable for HYSCR3rs {}
///`write(|w| ..)` method takes [`hyscr3::W`](W) writer structure
impl crate::Writable for HYSCR3rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets HYSCR3 to value 0
impl crate::Resettable for HYSCR3rs {}
