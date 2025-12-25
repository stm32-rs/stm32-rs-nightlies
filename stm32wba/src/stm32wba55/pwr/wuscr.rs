///Register `WUSCR` writer
pub type W = crate::W<WUSCRrs>;
///Field `CWUF1` writer - Clear wakeup flag 1 Access can be secured by PWR WUP1SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with PWR SPRIV or when non-secure with PWR NSPRIV. Writing 1 to this bit clears the WUF1 flag in PWR_WUSR.
pub type CWUF1_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CWUF2` writer - Clear wakeup flag 2 Access can be secured by PWR WUP2SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with PWR SPRIV or when non-secure with PWR NSPRIV. Writing 1 to this bit clears the WUF2 flag in PWR_WUSR.
pub type CWUF2_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CWUF3` writer - Clear wakeup flag 3 Access can be secured by PWR WUP3SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with PWR SPRIV or when non-secure with PWR NSPRIV. Writing 1 to this bit clears the WUF3 flag in PWR_WUSR.
pub type CWUF3_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CWUF4` writer - Clear wakeup flag 4 Access can be secured by PWR WUP4SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with PWR SPRIV or when non-secure with PWR NSPRIV. Writing 1 to this bit clears the WUF4 flag in PWR_WUSR.
pub type CWUF4_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CWUF5` writer - Clear wakeup flag 5 Access can be secured by PWR WUP5SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with PWR SPRIV or when non-secure with PWR NSPRIV. Writing 1 to this bit clears the WUF5 flag in PWR_WUSR.
pub type CWUF5_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CWUF6` writer - Clear wakeup flag 6 Access can be secured by PWR WUP6SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with PWR SPRIV or when non-secure with PWR NSPRIV. Writing 1 to this bit clears the WUF6 flag in PWR_WUSR.
pub type CWUF6_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CWUF7` writer - Clear wakeup flag 7 Access can be secured by PWR WUP7SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with PWR SPRIV or when non-secure with PWR NSPRIV. Writing 1 to this bit clears the WUF7 flag in PWR_WUSR.
pub type CWUF7_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CWUF8` writer - Clear wakeup flag 8 Access can be secured by PWR WUP8SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with PWR SPRIV or when non-secure with PWR NSPRIV. Writing 1 to this bit clears the WUF8 flag in PWR_WUSR.
pub type CWUF8_W<'a, REG> = crate::BitWriter<'a, REG>;
impl core::fmt::Debug for crate::generic::Reg<WUSCRrs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bit 0 - Clear wakeup flag 1 Access can be secured by PWR WUP1SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with PWR SPRIV or when non-secure with PWR NSPRIV. Writing 1 to this bit clears the WUF1 flag in PWR_WUSR.
    #[inline(always)]
    pub fn cwuf1(&mut self) -> CWUF1_W<'_, WUSCRrs> {
        CWUF1_W::new(self, 0)
    }
    ///Bit 1 - Clear wakeup flag 2 Access can be secured by PWR WUP2SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with PWR SPRIV or when non-secure with PWR NSPRIV. Writing 1 to this bit clears the WUF2 flag in PWR_WUSR.
    #[inline(always)]
    pub fn cwuf2(&mut self) -> CWUF2_W<'_, WUSCRrs> {
        CWUF2_W::new(self, 1)
    }
    ///Bit 2 - Clear wakeup flag 3 Access can be secured by PWR WUP3SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with PWR SPRIV or when non-secure with PWR NSPRIV. Writing 1 to this bit clears the WUF3 flag in PWR_WUSR.
    #[inline(always)]
    pub fn cwuf3(&mut self) -> CWUF3_W<'_, WUSCRrs> {
        CWUF3_W::new(self, 2)
    }
    ///Bit 3 - Clear wakeup flag 4 Access can be secured by PWR WUP4SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with PWR SPRIV or when non-secure with PWR NSPRIV. Writing 1 to this bit clears the WUF4 flag in PWR_WUSR.
    #[inline(always)]
    pub fn cwuf4(&mut self) -> CWUF4_W<'_, WUSCRrs> {
        CWUF4_W::new(self, 3)
    }
    ///Bit 4 - Clear wakeup flag 5 Access can be secured by PWR WUP5SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with PWR SPRIV or when non-secure with PWR NSPRIV. Writing 1 to this bit clears the WUF5 flag in PWR_WUSR.
    #[inline(always)]
    pub fn cwuf5(&mut self) -> CWUF5_W<'_, WUSCRrs> {
        CWUF5_W::new(self, 4)
    }
    ///Bit 5 - Clear wakeup flag 6 Access can be secured by PWR WUP6SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with PWR SPRIV or when non-secure with PWR NSPRIV. Writing 1 to this bit clears the WUF6 flag in PWR_WUSR.
    #[inline(always)]
    pub fn cwuf6(&mut self) -> CWUF6_W<'_, WUSCRrs> {
        CWUF6_W::new(self, 5)
    }
    ///Bit 6 - Clear wakeup flag 7 Access can be secured by PWR WUP7SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with PWR SPRIV or when non-secure with PWR NSPRIV. Writing 1 to this bit clears the WUF7 flag in PWR_WUSR.
    #[inline(always)]
    pub fn cwuf7(&mut self) -> CWUF7_W<'_, WUSCRrs> {
        CWUF7_W::new(self, 6)
    }
    ///Bit 7 - Clear wakeup flag 8 Access can be secured by PWR WUP8SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with PWR SPRIV or when non-secure with PWR NSPRIV. Writing 1 to this bit clears the WUF8 flag in PWR_WUSR.
    #[inline(always)]
    pub fn cwuf8(&mut self) -> CWUF8_W<'_, WUSCRrs> {
        CWUF8_W::new(self, 7)
    }
}
/**PWR wakeup status clear register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wuscr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WBA55.html#PWR:WUSCR)*/
pub struct WUSCRrs;
impl crate::RegisterSpec for WUSCRrs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`wuscr::W`](W) writer structure
impl crate::Writable for WUSCRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets WUSCR to value 0
impl crate::Resettable for WUSCRrs {}
