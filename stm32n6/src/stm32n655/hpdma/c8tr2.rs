///Register `C8TR2` reader
pub type R = crate::R<C8TR2rs>;
///Register `C8TR2` writer
pub type W = crate::W<C8TR2rs>;
///Field `REQSEL` reader - hardware request selection
pub type REQSEL_R = crate::FieldReader;
///Field `REQSEL` writer - hardware request selection
pub type REQSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `SWREQ` reader - software request
pub type SWREQ_R = crate::BitReader;
///Field `SWREQ` writer - software request
pub type SWREQ_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DREQ` reader - destination hardware request
pub type DREQ_R = crate::BitReader;
///Field `DREQ` writer - destination hardware request
pub type DREQ_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BREQ` reader - Block hardware request
pub type BREQ_R = crate::BitReader;
///Field `BREQ` writer - Block hardware request
pub type BREQ_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PFREQ` reader - Hardware request in peripheral flow control mode
pub type PFREQ_R = crate::BitReader;
///Field `PFREQ` writer - Hardware request in peripheral flow control mode
pub type PFREQ_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TRIGM` reader - trigger mode
pub type TRIGM_R = crate::FieldReader;
///Field `TRIGM` writer - trigger mode
pub type TRIGM_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `TRIGSEL` reader - trigger event input selection
pub type TRIGSEL_R = crate::FieldReader;
///Field `TRIGSEL` writer - trigger event input selection
pub type TRIGSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
///Field `TRIGPOL` reader - trigger event polarity
pub type TRIGPOL_R = crate::FieldReader;
///Field `TRIGPOL` writer - trigger event polarity
pub type TRIGPOL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `TCEM` reader - transfer complete event mode
pub type TCEM_R = crate::FieldReader;
///Field `TCEM` writer - transfer complete event mode
pub type TCEM_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    ///Bits 0:7 - hardware request selection
    #[inline(always)]
    pub fn reqsel(&self) -> REQSEL_R {
        REQSEL_R::new((self.bits & 0xff) as u8)
    }
    ///Bit 9 - software request
    #[inline(always)]
    pub fn swreq(&self) -> SWREQ_R {
        SWREQ_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - destination hardware request
    #[inline(always)]
    pub fn dreq(&self) -> DREQ_R {
        DREQ_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - Block hardware request
    #[inline(always)]
    pub fn breq(&self) -> BREQ_R {
        BREQ_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - Hardware request in peripheral flow control mode
    #[inline(always)]
    pub fn pfreq(&self) -> PFREQ_R {
        PFREQ_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bits 14:15 - trigger mode
    #[inline(always)]
    pub fn trigm(&self) -> TRIGM_R {
        TRIGM_R::new(((self.bits >> 14) & 3) as u8)
    }
    ///Bits 16:22 - trigger event input selection
    #[inline(always)]
    pub fn trigsel(&self) -> TRIGSEL_R {
        TRIGSEL_R::new(((self.bits >> 16) & 0x7f) as u8)
    }
    ///Bits 24:25 - trigger event polarity
    #[inline(always)]
    pub fn trigpol(&self) -> TRIGPOL_R {
        TRIGPOL_R::new(((self.bits >> 24) & 3) as u8)
    }
    ///Bits 30:31 - transfer complete event mode
    #[inline(always)]
    pub fn tcem(&self) -> TCEM_R {
        TCEM_R::new(((self.bits >> 30) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("C8TR2")
            .field("reqsel", &self.reqsel())
            .field("swreq", &self.swreq())
            .field("dreq", &self.dreq())
            .field("breq", &self.breq())
            .field("pfreq", &self.pfreq())
            .field("trigm", &self.trigm())
            .field("trigsel", &self.trigsel())
            .field("trigpol", &self.trigpol())
            .field("tcem", &self.tcem())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - hardware request selection
    #[inline(always)]
    pub fn reqsel(&mut self) -> REQSEL_W<'_, C8TR2rs> {
        REQSEL_W::new(self, 0)
    }
    ///Bit 9 - software request
    #[inline(always)]
    pub fn swreq(&mut self) -> SWREQ_W<'_, C8TR2rs> {
        SWREQ_W::new(self, 9)
    }
    ///Bit 10 - destination hardware request
    #[inline(always)]
    pub fn dreq(&mut self) -> DREQ_W<'_, C8TR2rs> {
        DREQ_W::new(self, 10)
    }
    ///Bit 11 - Block hardware request
    #[inline(always)]
    pub fn breq(&mut self) -> BREQ_W<'_, C8TR2rs> {
        BREQ_W::new(self, 11)
    }
    ///Bit 12 - Hardware request in peripheral flow control mode
    #[inline(always)]
    pub fn pfreq(&mut self) -> PFREQ_W<'_, C8TR2rs> {
        PFREQ_W::new(self, 12)
    }
    ///Bits 14:15 - trigger mode
    #[inline(always)]
    pub fn trigm(&mut self) -> TRIGM_W<'_, C8TR2rs> {
        TRIGM_W::new(self, 14)
    }
    ///Bits 16:22 - trigger event input selection
    #[inline(always)]
    pub fn trigsel(&mut self) -> TRIGSEL_W<'_, C8TR2rs> {
        TRIGSEL_W::new(self, 16)
    }
    ///Bits 24:25 - trigger event polarity
    #[inline(always)]
    pub fn trigpol(&mut self) -> TRIGPOL_W<'_, C8TR2rs> {
        TRIGPOL_W::new(self, 24)
    }
    ///Bits 30:31 - transfer complete event mode
    #[inline(always)]
    pub fn tcem(&mut self) -> TCEM_W<'_, C8TR2rs> {
        TCEM_W::new(self, 30)
    }
}
/**HPDMA channel 8 transfer register 2

You can [`read`](crate::Reg::read) this register and get [`c8tr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c8tr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#HPDMA:C8TR2)*/
pub struct C8TR2rs;
impl crate::RegisterSpec for C8TR2rs {
    type Ux = u32;
}
///`read()` method returns [`c8tr2::R`](R) reader structure
impl crate::Readable for C8TR2rs {}
///`write(|w| ..)` method takes [`c8tr2::W`](W) writer structure
impl crate::Writable for C8TR2rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets C8TR2 to value 0
impl crate::Resettable for C8TR2rs {}
