///Register `PRGITR` reader
pub type R = crate::R<PRGITRrs>;
///Register `PRGITR` writer
pub type W = crate::W<PRGITRrs>;
///Field `LB0VC` reader - Line/byte counter 0 linked to a virtual channel
pub type LB0VC_R = crate::FieldReader;
///Field `LB0VC` writer - Line/byte counter 0 linked to a virtual channel
pub type LB0VC_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `LB0EN` reader - Line/byte 0 counter enable
pub type LB0EN_R = crate::BitReader;
///Field `LB0EN` writer - Line/byte 0 counter enable
pub type LB0EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LB1VC` reader - Line/byte counter 1 linked to a virtual channel
pub type LB1VC_R = crate::FieldReader;
///Field `LB1VC` writer - Line/byte counter 1 linked to a virtual channel
pub type LB1VC_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `LB1EN` reader - Line/byte 1 counter enable
pub type LB1EN_R = crate::BitReader;
///Field `LB1EN` writer - Line/byte 1 counter enable
pub type LB1EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LB2VC` reader - Line/byte counter 2 linked to a virtual channel
pub type LB2VC_R = crate::FieldReader;
///Field `LB2VC` writer - Line/byte counter 2 linked to a virtual channel
pub type LB2VC_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `LB2EN` reader - Line/byte 2 counter enable
pub type LB2EN_R = crate::BitReader;
///Field `LB2EN` writer - Line/byte 2 counter enable
pub type LB2EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LB3VC` reader - Line/byte counter 3 linked to a virtual channel
pub type LB3VC_R = crate::FieldReader;
///Field `LB3VC` writer - Line/byte counter 3 linked to a virtual channel
pub type LB3VC_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `LB3EN` reader - Line/byte 3 counter enable
pub type LB3EN_R = crate::BitReader;
///Field `LB3EN` writer - Line/byte 3 counter enable
pub type LB3EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TIM0VC` reader - TIM0 base time linked to a virtual channel
pub type TIM0VC_R = crate::FieldReader;
///Field `TIM0VC` writer - TIM0 base time linked to a virtual channel
pub type TIM0VC_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `TIM0EOF` reader - TIM0 base time starting from the EOF
pub type TIM0EOF_R = crate::BitReader;
///Field `TIM0EOF` writer - TIM0 base time starting from the EOF
pub type TIM0EOF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TIM0EN` reader - TIM0 base time enable
pub type TIM0EN_R = crate::BitReader;
///Field `TIM0EN` writer - TIM0 base time enable
pub type TIM0EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TIM1VC` reader - TIM1 base time linked to a virtual channel
pub type TIM1VC_R = crate::FieldReader;
///Field `TIM1VC` writer - TIM1 base time linked to a virtual channel
pub type TIM1VC_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `TIM1EOF` reader - TIM1 base time starting from the EOF
pub type TIM1EOF_R = crate::BitReader;
///Field `TIM1EOF` writer - TIM1 base time starting from the EOF
pub type TIM1EOF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TIM1EN` reader - TIM1 base time enable
pub type TIM1EN_R = crate::BitReader;
///Field `TIM1EN` writer - TIM1 base time enable
pub type TIM1EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TIM2VC` reader - TIM2 base time linked to a virtual channel
pub type TIM2VC_R = crate::FieldReader;
///Field `TIM2VC` writer - TIM2 base time linked to a virtual channel
pub type TIM2VC_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `TIM2EOF` reader - TIM2 base time starting from the EOF
pub type TIM2EOF_R = crate::BitReader;
///Field `TIM2EOF` writer - TIM2 base time starting from the EOF
pub type TIM2EOF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TIM2EN` reader - TIM2 base time enable
pub type TIM2EN_R = crate::BitReader;
///Field `TIM2EN` writer - TIM2 base time enable
pub type TIM2EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TIM3VC` reader - TIM3 base time linked to a virtual channel
pub type TIM3VC_R = crate::FieldReader;
///Field `TIM3VC` writer - TIM3 base time linked to a virtual channel
pub type TIM3VC_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `TIM3EOF` reader - TIM3 base time starting from the EOF
pub type TIM3EOF_R = crate::BitReader;
///Field `TIM3EOF` writer - TIM3 base time starting from the EOF
pub type TIM3EOF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TIM3EN` reader - TIM3 base time enable
pub type TIM3EN_R = crate::BitReader;
///Field `TIM3EN` writer - TIM3 base time enable
pub type TIM3EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:1 - Line/byte counter 0 linked to a virtual channel
    #[inline(always)]
    pub fn lb0vc(&self) -> LB0VC_R {
        LB0VC_R::new((self.bits & 3) as u8)
    }
    ///Bit 3 - Line/byte 0 counter enable
    #[inline(always)]
    pub fn lb0en(&self) -> LB0EN_R {
        LB0EN_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bits 4:5 - Line/byte counter 1 linked to a virtual channel
    #[inline(always)]
    pub fn lb1vc(&self) -> LB1VC_R {
        LB1VC_R::new(((self.bits >> 4) & 3) as u8)
    }
    ///Bit 7 - Line/byte 1 counter enable
    #[inline(always)]
    pub fn lb1en(&self) -> LB1EN_R {
        LB1EN_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bits 8:9 - Line/byte counter 2 linked to a virtual channel
    #[inline(always)]
    pub fn lb2vc(&self) -> LB2VC_R {
        LB2VC_R::new(((self.bits >> 8) & 3) as u8)
    }
    ///Bit 11 - Line/byte 2 counter enable
    #[inline(always)]
    pub fn lb2en(&self) -> LB2EN_R {
        LB2EN_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bits 12:13 - Line/byte counter 3 linked to a virtual channel
    #[inline(always)]
    pub fn lb3vc(&self) -> LB3VC_R {
        LB3VC_R::new(((self.bits >> 12) & 3) as u8)
    }
    ///Bit 15 - Line/byte 3 counter enable
    #[inline(always)]
    pub fn lb3en(&self) -> LB3EN_R {
        LB3EN_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bits 16:17 - TIM0 base time linked to a virtual channel
    #[inline(always)]
    pub fn tim0vc(&self) -> TIM0VC_R {
        TIM0VC_R::new(((self.bits >> 16) & 3) as u8)
    }
    ///Bit 18 - TIM0 base time starting from the EOF
    #[inline(always)]
    pub fn tim0eof(&self) -> TIM0EOF_R {
        TIM0EOF_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - TIM0 base time enable
    #[inline(always)]
    pub fn tim0en(&self) -> TIM0EN_R {
        TIM0EN_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bits 20:21 - TIM1 base time linked to a virtual channel
    #[inline(always)]
    pub fn tim1vc(&self) -> TIM1VC_R {
        TIM1VC_R::new(((self.bits >> 20) & 3) as u8)
    }
    ///Bit 22 - TIM1 base time starting from the EOF
    #[inline(always)]
    pub fn tim1eof(&self) -> TIM1EOF_R {
        TIM1EOF_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - TIM1 base time enable
    #[inline(always)]
    pub fn tim1en(&self) -> TIM1EN_R {
        TIM1EN_R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bits 24:25 - TIM2 base time linked to a virtual channel
    #[inline(always)]
    pub fn tim2vc(&self) -> TIM2VC_R {
        TIM2VC_R::new(((self.bits >> 24) & 3) as u8)
    }
    ///Bit 26 - TIM2 base time starting from the EOF
    #[inline(always)]
    pub fn tim2eof(&self) -> TIM2EOF_R {
        TIM2EOF_R::new(((self.bits >> 26) & 1) != 0)
    }
    ///Bit 27 - TIM2 base time enable
    #[inline(always)]
    pub fn tim2en(&self) -> TIM2EN_R {
        TIM2EN_R::new(((self.bits >> 27) & 1) != 0)
    }
    ///Bits 28:29 - TIM3 base time linked to a virtual channel
    #[inline(always)]
    pub fn tim3vc(&self) -> TIM3VC_R {
        TIM3VC_R::new(((self.bits >> 28) & 3) as u8)
    }
    ///Bit 30 - TIM3 base time starting from the EOF
    #[inline(always)]
    pub fn tim3eof(&self) -> TIM3EOF_R {
        TIM3EOF_R::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - TIM3 base time enable
    #[inline(always)]
    pub fn tim3en(&self) -> TIM3EN_R {
        TIM3EN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PRGITR")
            .field("lb0vc", &self.lb0vc())
            .field("lb0en", &self.lb0en())
            .field("lb1vc", &self.lb1vc())
            .field("lb1en", &self.lb1en())
            .field("lb2vc", &self.lb2vc())
            .field("lb2en", &self.lb2en())
            .field("lb3vc", &self.lb3vc())
            .field("lb3en", &self.lb3en())
            .field("tim0vc", &self.tim0vc())
            .field("tim0eof", &self.tim0eof())
            .field("tim0en", &self.tim0en())
            .field("tim1vc", &self.tim1vc())
            .field("tim1eof", &self.tim1eof())
            .field("tim1en", &self.tim1en())
            .field("tim2vc", &self.tim2vc())
            .field("tim2eof", &self.tim2eof())
            .field("tim2en", &self.tim2en())
            .field("tim3vc", &self.tim3vc())
            .field("tim3eof", &self.tim3eof())
            .field("tim3en", &self.tim3en())
            .finish()
    }
}
impl W {
    ///Bits 0:1 - Line/byte counter 0 linked to a virtual channel
    #[inline(always)]
    pub fn lb0vc(&mut self) -> LB0VC_W<'_, PRGITRrs> {
        LB0VC_W::new(self, 0)
    }
    ///Bit 3 - Line/byte 0 counter enable
    #[inline(always)]
    pub fn lb0en(&mut self) -> LB0EN_W<'_, PRGITRrs> {
        LB0EN_W::new(self, 3)
    }
    ///Bits 4:5 - Line/byte counter 1 linked to a virtual channel
    #[inline(always)]
    pub fn lb1vc(&mut self) -> LB1VC_W<'_, PRGITRrs> {
        LB1VC_W::new(self, 4)
    }
    ///Bit 7 - Line/byte 1 counter enable
    #[inline(always)]
    pub fn lb1en(&mut self) -> LB1EN_W<'_, PRGITRrs> {
        LB1EN_W::new(self, 7)
    }
    ///Bits 8:9 - Line/byte counter 2 linked to a virtual channel
    #[inline(always)]
    pub fn lb2vc(&mut self) -> LB2VC_W<'_, PRGITRrs> {
        LB2VC_W::new(self, 8)
    }
    ///Bit 11 - Line/byte 2 counter enable
    #[inline(always)]
    pub fn lb2en(&mut self) -> LB2EN_W<'_, PRGITRrs> {
        LB2EN_W::new(self, 11)
    }
    ///Bits 12:13 - Line/byte counter 3 linked to a virtual channel
    #[inline(always)]
    pub fn lb3vc(&mut self) -> LB3VC_W<'_, PRGITRrs> {
        LB3VC_W::new(self, 12)
    }
    ///Bit 15 - Line/byte 3 counter enable
    #[inline(always)]
    pub fn lb3en(&mut self) -> LB3EN_W<'_, PRGITRrs> {
        LB3EN_W::new(self, 15)
    }
    ///Bits 16:17 - TIM0 base time linked to a virtual channel
    #[inline(always)]
    pub fn tim0vc(&mut self) -> TIM0VC_W<'_, PRGITRrs> {
        TIM0VC_W::new(self, 16)
    }
    ///Bit 18 - TIM0 base time starting from the EOF
    #[inline(always)]
    pub fn tim0eof(&mut self) -> TIM0EOF_W<'_, PRGITRrs> {
        TIM0EOF_W::new(self, 18)
    }
    ///Bit 19 - TIM0 base time enable
    #[inline(always)]
    pub fn tim0en(&mut self) -> TIM0EN_W<'_, PRGITRrs> {
        TIM0EN_W::new(self, 19)
    }
    ///Bits 20:21 - TIM1 base time linked to a virtual channel
    #[inline(always)]
    pub fn tim1vc(&mut self) -> TIM1VC_W<'_, PRGITRrs> {
        TIM1VC_W::new(self, 20)
    }
    ///Bit 22 - TIM1 base time starting from the EOF
    #[inline(always)]
    pub fn tim1eof(&mut self) -> TIM1EOF_W<'_, PRGITRrs> {
        TIM1EOF_W::new(self, 22)
    }
    ///Bit 23 - TIM1 base time enable
    #[inline(always)]
    pub fn tim1en(&mut self) -> TIM1EN_W<'_, PRGITRrs> {
        TIM1EN_W::new(self, 23)
    }
    ///Bits 24:25 - TIM2 base time linked to a virtual channel
    #[inline(always)]
    pub fn tim2vc(&mut self) -> TIM2VC_W<'_, PRGITRrs> {
        TIM2VC_W::new(self, 24)
    }
    ///Bit 26 - TIM2 base time starting from the EOF
    #[inline(always)]
    pub fn tim2eof(&mut self) -> TIM2EOF_W<'_, PRGITRrs> {
        TIM2EOF_W::new(self, 26)
    }
    ///Bit 27 - TIM2 base time enable
    #[inline(always)]
    pub fn tim2en(&mut self) -> TIM2EN_W<'_, PRGITRrs> {
        TIM2EN_W::new(self, 27)
    }
    ///Bits 28:29 - TIM3 base time linked to a virtual channel
    #[inline(always)]
    pub fn tim3vc(&mut self) -> TIM3VC_W<'_, PRGITRrs> {
        TIM3VC_W::new(self, 28)
    }
    ///Bit 30 - TIM3 base time starting from the EOF
    #[inline(always)]
    pub fn tim3eof(&mut self) -> TIM3EOF_W<'_, PRGITRrs> {
        TIM3EOF_W::new(self, 30)
    }
    ///Bit 31 - TIM3 base time enable
    #[inline(always)]
    pub fn tim3en(&mut self) -> TIM3EN_W<'_, PRGITRrs> {
        TIM3EN_W::new(self, 31)
    }
}
/**CSI-2 Host program interrupt register

You can [`read`](crate::Reg::read) this register and get [`prgitr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`prgitr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#CSI:PRGITR)*/
pub struct PRGITRrs;
impl crate::RegisterSpec for PRGITRrs {
    type Ux = u32;
}
///`read()` method returns [`prgitr::R`](R) reader structure
impl crate::Readable for PRGITRrs {}
///`write(|w| ..)` method takes [`prgitr::W`](W) writer structure
impl crate::Writable for PRGITRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets PRGITR to value 0
impl crate::Resettable for PRGITRrs {}
