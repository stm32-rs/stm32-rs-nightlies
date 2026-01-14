///Register `P0SCSTR` reader
pub type R = crate::R<P0SCSTRrs>;
///Register `P0SCSTR` writer
pub type W = crate::W<P0SCSTRrs>;
///Field `HSTART` reader - Horizontal start, from 0 to 4094 words wide
pub type HSTART_R = crate::FieldReader<u16>;
///Field `HSTART` writer - Horizontal start, from 0 to 4094 words wide
pub type HSTART_W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
///Field `VSTART` reader - Vertical start, from 0 to 4094 pixels high
pub type VSTART_R = crate::FieldReader<u16>;
///Field `VSTART` writer - Vertical start, from 0 to 4094 pixels high
pub type VSTART_W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    ///Bits 0:11 - Horizontal start, from 0 to 4094 words wide
    #[inline(always)]
    pub fn hstart(&self) -> HSTART_R {
        HSTART_R::new((self.bits & 0x0fff) as u16)
    }
    ///Bits 16:27 - Vertical start, from 0 to 4094 pixels high
    #[inline(always)]
    pub fn vstart(&self) -> VSTART_R {
        VSTART_R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("P0SCSTR")
            .field("hstart", &self.hstart())
            .field("vstart", &self.vstart())
            .finish()
    }
}
impl W {
    ///Bits 0:11 - Horizontal start, from 0 to 4094 words wide
    #[inline(always)]
    pub fn hstart(&mut self) -> HSTART_W<'_, P0SCSTRrs> {
        HSTART_W::new(self, 0)
    }
    ///Bits 16:27 - Vertical start, from 0 to 4094 pixels high
    #[inline(always)]
    pub fn vstart(&mut self) -> VSTART_W<'_, P0SCSTRrs> {
        VSTART_W::new(self, 16)
    }
}
/**DCMIPP Pipe0 stat/crop start register

You can [`read`](crate::Reg::read) this register and get [`p0scstr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p0scstr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7S.html#DCMIPP:P0SCSTR)*/
pub struct P0SCSTRrs;
impl crate::RegisterSpec for P0SCSTRrs {
    type Ux = u32;
}
///`read()` method returns [`p0scstr::R`](R) reader structure
impl crate::Readable for P0SCSTRrs {}
///`write(|w| ..)` method takes [`p0scstr::W`](W) writer structure
impl crate::Writable for P0SCSTRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets P0SCSTR to value 0
impl crate::Resettable for P0SCSTRrs {}
