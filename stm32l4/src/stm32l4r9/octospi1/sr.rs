///Register `SR` reader
pub type R = crate::R<SRrs>;
///Register `SR` writer
pub type W = crate::W<SRrs>;
///Field `TEF` reader - Transfer error flag
pub type TEF_R = crate::BitReader;
///Field `TEF` writer - Transfer error flag
pub type TEF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TCF` reader - Transfer complete flag
pub type TCF_R = crate::BitReader;
///Field `TCF` writer - Transfer complete flag
pub type TCF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FTF` reader - FIFO threshold flag
pub type FTF_R = crate::BitReader;
///Field `FTF` writer - FIFO threshold flag
pub type FTF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SMF` reader - Status match flag
pub type SMF_R = crate::BitReader;
///Field `SMF` writer - Status match flag
pub type SMF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TOF` reader - Timeout flag
pub type TOF_R = crate::BitReader;
///Field `TOF` writer - Timeout flag
pub type TOF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BUSY` reader - BUSY
pub type BUSY_R = crate::BitReader;
///Field `BUSY` writer - BUSY
pub type BUSY_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FLEVEL` reader - FIFO level
pub type FLEVEL_R = crate::FieldReader;
///Field `FLEVEL` writer - FIFO level
pub type FLEVEL_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    ///Bit 0 - Transfer error flag
    #[inline(always)]
    pub fn tef(&self) -> TEF_R {
        TEF_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Transfer complete flag
    #[inline(always)]
    pub fn tcf(&self) -> TCF_R {
        TCF_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - FIFO threshold flag
    #[inline(always)]
    pub fn ftf(&self) -> FTF_R {
        FTF_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Status match flag
    #[inline(always)]
    pub fn smf(&self) -> SMF_R {
        SMF_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Timeout flag
    #[inline(always)]
    pub fn tof(&self) -> TOF_R {
        TOF_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - BUSY
    #[inline(always)]
    pub fn busy(&self) -> BUSY_R {
        BUSY_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bits 8:13 - FIFO level
    #[inline(always)]
    pub fn flevel(&self) -> FLEVEL_R {
        FLEVEL_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SR")
            .field("tef", &self.tef())
            .field("tcf", &self.tcf())
            .field("ftf", &self.ftf())
            .field("smf", &self.smf())
            .field("tof", &self.tof())
            .field("busy", &self.busy())
            .field("flevel", &self.flevel())
            .finish()
    }
}
impl W {
    ///Bit 0 - Transfer error flag
    #[inline(always)]
    pub fn tef(&mut self) -> TEF_W<SRrs> {
        TEF_W::new(self, 0)
    }
    ///Bit 1 - Transfer complete flag
    #[inline(always)]
    pub fn tcf(&mut self) -> TCF_W<SRrs> {
        TCF_W::new(self, 1)
    }
    ///Bit 2 - FIFO threshold flag
    #[inline(always)]
    pub fn ftf(&mut self) -> FTF_W<SRrs> {
        FTF_W::new(self, 2)
    }
    ///Bit 3 - Status match flag
    #[inline(always)]
    pub fn smf(&mut self) -> SMF_W<SRrs> {
        SMF_W::new(self, 3)
    }
    ///Bit 4 - Timeout flag
    #[inline(always)]
    pub fn tof(&mut self) -> TOF_W<SRrs> {
        TOF_W::new(self, 4)
    }
    ///Bit 5 - BUSY
    #[inline(always)]
    pub fn busy(&mut self) -> BUSY_W<SRrs> {
        BUSY_W::new(self, 5)
    }
    ///Bits 8:13 - FIFO level
    #[inline(always)]
    pub fn flevel(&mut self) -> FLEVEL_W<SRrs> {
        FLEVEL_W::new(self, 8)
    }
}
/**status register

You can [`read`](crate::Reg::read) this register and get [`sr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4R9.html#OCTOSPI1:SR)*/
pub struct SRrs;
impl crate::RegisterSpec for SRrs {
    type Ux = u32;
}
///`read()` method returns [`sr::R`](R) reader structure
impl crate::Readable for SRrs {}
///`write(|w| ..)` method takes [`sr::W`](W) writer structure
impl crate::Writable for SRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets SR to value 0
impl crate::Resettable for SRrs {
    const RESET_VALUE: u32 = 0;
}
