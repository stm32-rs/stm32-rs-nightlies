///Register `I3C_GETCAPR` reader
pub type R = crate::R<I3C_GETCAPRrs>;
///Register `I3C_GETCAPR` writer
pub type W = crate::W<I3C_GETCAPRrs>;
/**Field `CAPPEND` reader - IBI MDB support for pending read notification This bit is written by software during bus initialization (i.e. I3C_CFGR.EN=0) and indicates the support (or not) of the pending read notification via the IBI MDB\[7:0\]
value. This bit is used to return the GETCAP3 byte in response to the GETCAPS CCC format 1.*/
pub type CAPPEND_R = crate::BitReader;
/**Field `CAPPEND` writer - IBI MDB support for pending read notification This bit is written by software during bus initialization (i.e. I3C_CFGR.EN=0) and indicates the support (or not) of the pending read notification via the IBI MDB\[7:0\]
value. This bit is used to return the GETCAP3 byte in response to the GETCAPS CCC format 1.*/
pub type CAPPEND_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    /**Bit 14 - IBI MDB support for pending read notification This bit is written by software during bus initialization (i.e. I3C_CFGR.EN=0) and indicates the support (or not) of the pending read notification via the IBI MDB\[7:0\]
    value. This bit is used to return the GETCAP3 byte in response to the GETCAPS CCC format 1.*/
    #[inline(always)]
    pub fn cappend(&self) -> CAPPEND_R {
        CAPPEND_R::new(((self.bits >> 14) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("I3C_GETCAPR")
            .field("cappend", &self.cappend())
            .finish()
    }
}
impl W {
    /**Bit 14 - IBI MDB support for pending read notification This bit is written by software during bus initialization (i.e. I3C_CFGR.EN=0) and indicates the support (or not) of the pending read notification via the IBI MDB\[7:0\]
    value. This bit is used to return the GETCAP3 byte in response to the GETCAPS CCC format 1.*/
    #[inline(always)]
    pub fn cappend(&mut self) -> CAPPEND_W<I3C_GETCAPRrs> {
        CAPPEND_W::new(self, 14)
    }
}
/**I3C get capability register

You can [`read`](crate::Reg::read) this register and get [`i3c_getcapr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i3c_getcapr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H503.html#I3C1:I3C_GETCAPR)*/
pub struct I3C_GETCAPRrs;
impl crate::RegisterSpec for I3C_GETCAPRrs {
    type Ux = u32;
}
///`read()` method returns [`i3c_getcapr::R`](R) reader structure
impl crate::Readable for I3C_GETCAPRrs {}
///`write(|w| ..)` method takes [`i3c_getcapr::W`](W) writer structure
impl crate::Writable for I3C_GETCAPRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets I3C_GETCAPR to value 0
impl crate::Resettable for I3C_GETCAPRrs {
    const RESET_VALUE: u32 = 0;
}
