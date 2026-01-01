///Register `P1RI2CR1` reader
pub type R = crate::R<P1RI2CR1rs>;
///Register `P1RI2CR1` writer
pub type W = crate::W<P1RI2CR1rs>;
///Field `HSTART` reader - Horizontal start, from 0 to 4094 pixels wide
pub type HSTART_R = crate::FieldReader<u16>;
///Field `HSTART` writer - Horizontal start, from 0 to 4094 pixels wide
pub type HSTART_W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
///Field `CLB` reader - Color line blue
pub type CLB_R = crate::FieldReader;
///Field `CLB` writer - Color line blue
pub type CLB_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `CLG` reader - Color line green
pub type CLG_R = crate::FieldReader;
///Field `CLG` writer - Color line green
pub type CLG_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `VSTART` reader - Vertical start, from 0 to 4094 pixels high
pub type VSTART_R = crate::FieldReader<u16>;
///Field `VSTART` writer - Vertical start, from 0 to 4094 pixels high
pub type VSTART_W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
///Field `CLR` reader - Color line red
pub type CLR_R = crate::FieldReader;
///Field `CLR` writer - Color line red
pub type CLR_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    ///Bits 0:11 - Horizontal start, from 0 to 4094 pixels wide
    #[inline(always)]
    pub fn hstart(&self) -> HSTART_R {
        HSTART_R::new((self.bits & 0x0fff) as u16)
    }
    ///Bits 12:13 - Color line blue
    #[inline(always)]
    pub fn clb(&self) -> CLB_R {
        CLB_R::new(((self.bits >> 12) & 3) as u8)
    }
    ///Bits 14:15 - Color line green
    #[inline(always)]
    pub fn clg(&self) -> CLG_R {
        CLG_R::new(((self.bits >> 14) & 3) as u8)
    }
    ///Bits 16:27 - Vertical start, from 0 to 4094 pixels high
    #[inline(always)]
    pub fn vstart(&self) -> VSTART_R {
        VSTART_R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
    ///Bits 28:29 - Color line red
    #[inline(always)]
    pub fn clr(&self) -> CLR_R {
        CLR_R::new(((self.bits >> 28) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("P1RI2CR1")
            .field("hstart", &self.hstart())
            .field("clb", &self.clb())
            .field("clg", &self.clg())
            .field("vstart", &self.vstart())
            .field("clr", &self.clr())
            .finish()
    }
}
impl W {
    ///Bits 0:11 - Horizontal start, from 0 to 4094 pixels wide
    #[inline(always)]
    pub fn hstart(&mut self) -> HSTART_W<'_, P1RI2CR1rs> {
        HSTART_W::new(self, 0)
    }
    ///Bits 12:13 - Color line blue
    #[inline(always)]
    pub fn clb(&mut self) -> CLB_W<'_, P1RI2CR1rs> {
        CLB_W::new(self, 12)
    }
    ///Bits 14:15 - Color line green
    #[inline(always)]
    pub fn clg(&mut self) -> CLG_W<'_, P1RI2CR1rs> {
        CLG_W::new(self, 14)
    }
    ///Bits 16:27 - Vertical start, from 0 to 4094 pixels high
    #[inline(always)]
    pub fn vstart(&mut self) -> VSTART_W<'_, P1RI2CR1rs> {
        VSTART_W::new(self, 16)
    }
    ///Bits 28:29 - Color line red
    #[inline(always)]
    pub fn clr(&mut self) -> CLR_W<'_, P1RI2CR1rs> {
        CLR_W::new(self, 28)
    }
}
/**DCMIPP Pipe1 ROI2 configuration register 1

You can [`read`](crate::Reg::read) this register and get [`p1ri2cr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p1ri2cr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#DCMIPP:P1RI2CR1)*/
pub struct P1RI2CR1rs;
impl crate::RegisterSpec for P1RI2CR1rs {
    type Ux = u32;
}
///`read()` method returns [`p1ri2cr1::R`](R) reader structure
impl crate::Readable for P1RI2CR1rs {}
///`write(|w| ..)` method takes [`p1ri2cr1::W`](W) writer structure
impl crate::Writable for P1RI2CR1rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets P1RI2CR1 to value 0
impl crate::Resettable for P1RI2CR1rs {}
