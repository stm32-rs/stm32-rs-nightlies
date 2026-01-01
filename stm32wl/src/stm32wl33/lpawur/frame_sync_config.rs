///Register `FRAME_SYNC_CONFIG` reader
pub type R = crate::R<FRAME_SYNC_CONFIGrs>;
///Register `FRAME_SYNC_CONFIG` writer
pub type W = crate::W<FRAME_SYNC_CONFIGrs>;
///Field `FRAME_SYNC_PATTERN_L` reader - The value of the frame sync pattern, Low word, manchester encoded, used when the frame sync length is 16 bit (default 0x9696 which represent a frame sync value of 0x99)
pub type FRAME_SYNC_PATTERN_L_R = crate::FieldReader<u16>;
///Field `FRAME_SYNC_PATTERN_L` writer - The value of the frame sync pattern, Low word, manchester encoded, used when the frame sync length is 16 bit (default 0x9696 which represent a frame sync value of 0x99)
pub type FRAME_SYNC_PATTERN_L_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
///Field `FRAME_SYNC_PATTERN_H` reader - The value of the frame sync pattern, High word, manchester encoded, used only when the frame sync length is 32 bits (default 0x0000 )
pub type FRAME_SYNC_PATTERN_H_R = crate::FieldReader<u16>;
///Field `FRAME_SYNC_PATTERN_H` writer - The value of the frame sync pattern, High word, manchester encoded, used only when the frame sync length is 32 bits (default 0x0000 )
pub type FRAME_SYNC_PATTERN_H_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    ///Bits 0:15 - The value of the frame sync pattern, Low word, manchester encoded, used when the frame sync length is 16 bit (default 0x9696 which represent a frame sync value of 0x99)
    #[inline(always)]
    pub fn frame_sync_pattern_l(&self) -> FRAME_SYNC_PATTERN_L_R {
        FRAME_SYNC_PATTERN_L_R::new((self.bits & 0xffff) as u16)
    }
    ///Bits 16:31 - The value of the frame sync pattern, High word, manchester encoded, used only when the frame sync length is 32 bits (default 0x0000 )
    #[inline(always)]
    pub fn frame_sync_pattern_h(&self) -> FRAME_SYNC_PATTERN_H_R {
        FRAME_SYNC_PATTERN_H_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FRAME_SYNC_CONFIG")
            .field("frame_sync_pattern_l", &self.frame_sync_pattern_l())
            .field("frame_sync_pattern_h", &self.frame_sync_pattern_h())
            .finish()
    }
}
impl W {
    ///Bits 0:15 - The value of the frame sync pattern, Low word, manchester encoded, used when the frame sync length is 16 bit (default 0x9696 which represent a frame sync value of 0x99)
    #[inline(always)]
    pub fn frame_sync_pattern_l(&mut self) -> FRAME_SYNC_PATTERN_L_W<'_, FRAME_SYNC_CONFIGrs> {
        FRAME_SYNC_PATTERN_L_W::new(self, 0)
    }
    ///Bits 16:31 - The value of the frame sync pattern, High word, manchester encoded, used only when the frame sync length is 32 bits (default 0x0000 )
    #[inline(always)]
    pub fn frame_sync_pattern_h(&mut self) -> FRAME_SYNC_PATTERN_H_W<'_, FRAME_SYNC_CONFIGrs> {
        FRAME_SYNC_PATTERN_H_W::new(self, 16)
    }
}
/**FRAME_SYNC_CONFIG register

You can [`read`](crate::Reg::read) this register and get [`frame_sync_config::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`frame_sync_config::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#LPAWUR:FRAME_SYNC_CONFIG)*/
pub struct FRAME_SYNC_CONFIGrs;
impl crate::RegisterSpec for FRAME_SYNC_CONFIGrs {
    type Ux = u32;
}
///`read()` method returns [`frame_sync_config::R`](R) reader structure
impl crate::Readable for FRAME_SYNC_CONFIGrs {}
///`write(|w| ..)` method takes [`frame_sync_config::W`](W) writer structure
impl crate::Writable for FRAME_SYNC_CONFIGrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets FRAME_SYNC_CONFIG to value 0x9696
impl crate::Resettable for FRAME_SYNC_CONFIGrs {
    const RESET_VALUE: u32 = 0x9696;
}
