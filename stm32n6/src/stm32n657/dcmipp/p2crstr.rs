///Register `P2CRSTR` reader
pub type R = crate::R<P2CRSTRrs>;
///Register `P2CRSTR` writer
pub type W = crate::W<P2CRSTRrs>;
///Field `HSTART` reader - Horizontal start, from 0 to 4094 pixels wide
pub type HSTART_R = crate::FieldReader<u16>;
///Field `HSTART` writer - Horizontal start, from 0 to 4094 pixels wide
pub type HSTART_W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
///Field `VSTART` reader - Vertical start, from 0 to 4094 pixels high
pub type VSTART_R = crate::FieldReader<u16>;
///Field `VSTART` writer - Vertical start, from 0 to 4094 pixels high
pub type VSTART_W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    ///Bits 0:11 - Horizontal start, from 0 to 4094 pixels wide
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
        f.debug_struct("P2CRSTR")
            .field("hstart", &self.hstart())
            .field("vstart", &self.vstart())
            .finish()
    }
}
impl W {
    ///Bits 0:11 - Horizontal start, from 0 to 4094 pixels wide
    #[inline(always)]
    pub fn hstart(&mut self) -> HSTART_W<'_, P2CRSTRrs> {
        HSTART_W::new(self, 0)
    }
    ///Bits 16:27 - Vertical start, from 0 to 4094 pixels high
    #[inline(always)]
    pub fn vstart(&mut self) -> VSTART_W<'_, P2CRSTRrs> {
        VSTART_W::new(self, 16)
    }
}
/**DCMIPP Pipex crop window start register

You can [`read`](crate::Reg::read) this register and get [`p2crstr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p2crstr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DCMIPP:P2CRSTR)*/
pub struct P2CRSTRrs;
impl crate::RegisterSpec for P2CRSTRrs {
    type Ux = u32;
}
///`read()` method returns [`p2crstr::R`](R) reader structure
impl crate::Readable for P2CRSTRrs {}
///`write(|w| ..)` method takes [`p2crstr::W`](W) writer structure
impl crate::Writable for P2CRSTRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets P2CRSTR to value 0
impl crate::Resettable for P2CRSTRrs {}
