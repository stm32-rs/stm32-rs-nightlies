///Register `P2CRI7CR1` reader
pub type R = crate::R<P2CRI7CR1rs>;
///Field `HSTART` reader - Current horizontal start, from 0 to 4094 pixels wide
pub type HSTART_R = crate::FieldReader<u16>;
///Field `CLB` reader - Current color line blue
pub type CLB_R = crate::FieldReader;
///Field `CLG` reader - Current color line green
pub type CLG_R = crate::FieldReader;
///Field `VSTART` reader - Current vertical start, from 0 to 4094 pixels high
pub type VSTART_R = crate::FieldReader<u16>;
///Field `CLR` reader - Current color line red
pub type CLR_R = crate::FieldReader;
impl R {
    ///Bits 0:11 - Current horizontal start, from 0 to 4094 pixels wide
    #[inline(always)]
    pub fn hstart(&self) -> HSTART_R {
        HSTART_R::new((self.bits & 0x0fff) as u16)
    }
    ///Bits 12:13 - Current color line blue
    #[inline(always)]
    pub fn clb(&self) -> CLB_R {
        CLB_R::new(((self.bits >> 12) & 3) as u8)
    }
    ///Bits 14:15 - Current color line green
    #[inline(always)]
    pub fn clg(&self) -> CLG_R {
        CLG_R::new(((self.bits >> 14) & 3) as u8)
    }
    ///Bits 16:27 - Current vertical start, from 0 to 4094 pixels high
    #[inline(always)]
    pub fn vstart(&self) -> VSTART_R {
        VSTART_R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
    ///Bits 28:29 - Current color line red
    #[inline(always)]
    pub fn clr(&self) -> CLR_R {
        CLR_R::new(((self.bits >> 28) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("P2CRI7CR1")
            .field("hstart", &self.hstart())
            .field("clb", &self.clb())
            .field("clg", &self.clg())
            .field("vstart", &self.vstart())
            .field("clr", &self.clr())
            .finish()
    }
}
/**DCMIPP Pipe2 current ROI7 configuration register 1

You can [`read`](crate::Reg::read) this register and get [`p2cri7cr1::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DCMIPP:P2CRI7CR1)*/
pub struct P2CRI7CR1rs;
impl crate::RegisterSpec for P2CRI7CR1rs {
    type Ux = u32;
}
///`read()` method returns [`p2cri7cr1::R`](R) reader structure
impl crate::Readable for P2CRI7CR1rs {}
///`reset()` method sets P2CRI7CR1 to value 0
impl crate::Resettable for P2CRI7CR1rs {}
