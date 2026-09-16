///Register `ESR` reader
pub type R = crate::R<ESRrs>;
///Register `ESR` writer
pub type W = crate::W<ESRrs>;
///Field `EWGF` reader - EWGF
pub type EWGF_R = crate::BitReader;
///Field `EPVF` reader - EPVF
pub type EPVF_R = crate::BitReader;
///Field `BOFF` reader - BOFF
pub type BOFF_R = crate::BitReader;
///Field `LEC` reader - LEC
pub type LEC_R = crate::FieldReader;
///Field `LEC` writer - LEC
pub type LEC_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `TEC` reader - TEC
pub type TEC_R = crate::FieldReader;
///Field `REC` reader - REC
pub type REC_R = crate::FieldReader;
impl R {
    ///Bit 0 - EWGF
    #[inline(always)]
    pub fn ewgf(&self) -> EWGF_R {
        EWGF_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - EPVF
    #[inline(always)]
    pub fn epvf(&self) -> EPVF_R {
        EPVF_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - BOFF
    #[inline(always)]
    pub fn boff(&self) -> BOFF_R {
        BOFF_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bits 4:6 - LEC
    #[inline(always)]
    pub fn lec(&self) -> LEC_R {
        LEC_R::new(((self.bits >> 4) & 7) as u8)
    }
    ///Bits 16:23 - TEC
    #[inline(always)]
    pub fn tec(&self) -> TEC_R {
        TEC_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    ///Bits 24:31 - REC
    #[inline(always)]
    pub fn rec(&self) -> REC_R {
        REC_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ESR")
            .field("rec", &self.rec())
            .field("tec", &self.tec())
            .field("lec", &self.lec())
            .field("boff", &self.boff())
            .field("epvf", &self.epvf())
            .field("ewgf", &self.ewgf())
            .finish()
    }
}
impl W {
    ///Bits 4:6 - LEC
    #[inline(always)]
    pub fn lec(&mut self) -> LEC_W<'_, ESRrs> {
        LEC_W::new(self, 4)
    }
}
/**interrupt enable register

You can [`read`](crate::Reg::read) this register and get [`esr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`esr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F415.html#CAN1:ESR)*/
pub struct ESRrs;
impl crate::RegisterSpec for ESRrs {
    type Ux = u32;
}
///`read()` method returns [`esr::R`](R) reader structure
impl crate::Readable for ESRrs {}
///`write(|w| ..)` method takes [`esr::W`](W) writer structure
impl crate::Writable for ESRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets ESR to value 0
impl crate::Resettable for ESRrs {}
